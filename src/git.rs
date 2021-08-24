use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{FetchOptions, Progress, RemoteCallbacks};
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

fn print_clone_progress(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    let co_pct = if state.total > 0 {
        (100 * state.current) / state.total
    } else {
        0
    };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!();
            state.newline = true;
        }
        print!(
            "Resolving deltas {}/{}\r",
            stats.indexed_deltas(),
            stats.total_deltas()
        );
    } else {
        print!(
            "net {:3}% ({:4} kb, {:5}/{:5})  /  idx {:3}% ({:5}/{:5})  \
             /  chk {:3}% ({:4}/{:4}) {}\r",
            network_pct,
            kbytes,
            stats.received_objects(),
            stats.total_objects(),
            index_pct,
            stats.indexed_objects(),
            stats.total_objects(),
            co_pct,
            state.current,
            state.total,
            state
                .path
                .as_ref()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_default()
        )
    }
    io::stdout().flush().unwrap();
}

pub(crate) fn clone(remote: &str, path: &Path) -> Result<(), git2::Error> {
    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
        newline: false,
    });
    let mut cb = RemoteCallbacks::new();
    cb.credentials(git_credentials_callback);
    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print_clone_progress(&mut *state);
        true
    });

    let mut co = CheckoutBuilder::new();
    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        print_clone_progress(&mut *state);
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(cb);
    RepoBuilder::new()
        .fetch_options(fo)
        .with_checkout(co)
        .clone(remote, path)?;
    println!();

    Ok(())
}

fn git_credentials_callback(
    _url: &str,
    user_from_url: Option<&str>,
    cred_types_allowed: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
    let user = user_from_url.unwrap();

    if cred_types_allowed.contains(git2::CredentialType::SSH_KEY) {
        let private_key = dirs::home_dir().unwrap().join(".ssh").join("id_rsa");
        let cred = git2::Cred::ssh_key(user, None, &private_key, None);
        return cred;
    }

    Err(git2::Error::from_str("no credential option available"))
}
