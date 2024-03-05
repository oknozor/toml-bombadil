# Bats test

[Bats](https://github.com/bats-core/bats-core) is a bash testing framework. Toml bombadil use it in docker to write 
realistic test cases without breaking your workstation dotfiles.

1. Write a new bat test in [tests.bat](tests.sh) for example : 
    ```sh
    @test "Print version" {
      run bombadil --version
      assert_success
    }
    ```
   If you need to add some dotfiles to the docker user home, everything resides in `tom_home`. 
   
2. Build the test container `docker build -t bombadil-tests ./`
3. Run it `docker run -it bombadil-tests` : 

```
❯ docker run -it bombadil-tests
 ✓ Print help
 ✓ Print version
 ✓ Symlink bombadil config
 ✓ Links dots

4 tests, 0 failures
```

4. Submit a PR to the current release candidate branch.