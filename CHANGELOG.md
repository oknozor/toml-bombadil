# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [4.0.0](https://github.com/oknozor/toml-bombadil/compare/3.1.0..4.0.0) - 2024-10-23
#### Bug Fixes
- **(dot)** fix traversal bug with non utf8 path - ([ab7163e](https://github.com/oknozor/toml-bombadil/commit/ab7163ed0bb2172bef6cd3bff2173d00c85f8686)) - [@oknozor](https://github.com/oknozor)
- **(watch)** detech changes on .dot directory - ([9f19352](https://github.com/oknozor/toml-bombadil/commit/9f19352aab30ff934e7d50d22e7c041b1744861a)) - [@oknozor](https://github.com/oknozor)
- **(website)** fix toc ordering - ([100372e](https://github.com/oknozor/toml-bombadil/commit/100372efa87d2628a1c36a7f1368665d749545df)) - [@oknozor](https://github.com/oknozor)
- fix shellexpand again - ([909cb55](https://github.com/oknozor/toml-bombadil/commit/909cb551abc855f7c212603d1a4c6796def72985)) - [@oknozor](https://github.com/oknozor)
- expand tilde in dotfile paths - ([27e1537](https://github.com/oknozor/toml-bombadil/commit/27e153740d28af9a662f59cf79ef16c975cfe6dc)) - [@oknozor](https://github.com/oknozor)
- fix ssl issues with libgit2 - ([3c3617e](https://github.com/oknozor/toml-bombadil/commit/3c3617e9f0ba2d0dc2667363a057d5772bdf76e2)) - [@oknozor](https://github.com/oknozor)
- fix adding secrets producing invalid toml - ([58975fd](https://github.com/oknozor/toml-bombadil/commit/58975fdf9bc54bb27eb595b5bcca3edc205a922a)) - [@oknozor](https://github.com/oknozor)
- fix bats test and gpg secrets - ([29a64ec](https://github.com/oknozor/toml-bombadil/commit/29a64ec5ffe2cf5988a66333b87ff88390bb9e60)) - [@oknozor](https://github.com/oknozor)
- make install not remove current config on failure - ([0d13c92](https://github.com/oknozor/toml-bombadil/commit/0d13c9263f8e7cf6a2b7e95726bdb2820ed92fc4)) - [@oknozor](https://github.com/oknozor)
- fix ignored path - ([e47976e](https://github.com/oknozor/toml-bombadil/commit/e47976edbb96551c45fb9a4660a0aa2a8f68e621)) - [@oknozor](https://github.com/oknozor)
- bump watchexec to 2.0.0 - ([8bc5e9a](https://github.com/oknozor/toml-bombadil/commit/8bc5e9ab2d3b0604b2f1b1b519f74f3d68048980)) - [@oknozor](https://github.com/oknozor)
- display undeclared variables when rendering templates - ([28ad6c2](https://github.com/oknozor/toml-bombadil/commit/28ad6c2d0547d747ebcc248b4e51e0cbd215fd60)) - [@oknozor](https://github.com/oknozor)
- run cargo bump early in cog.toml so Cargo.lock is updated after the release build - ([352d718](https://github.com/oknozor/toml-bombadil/commit/352d7180d500fb95d73299b1b23bbec1ac6e871f)) - [@oknozor](https://github.com/oknozor)
- fix release github action workflow - ([df42705](https://github.com/oknozor/toml-bombadil/commit/df42705e939da4d084512e147ef347e42703ba6e)) - [@oknozor](https://github.com/oknozor)
- fix release version job output again - ([9cdca4b](https://github.com/oknozor/toml-bombadil/commit/9cdca4b76da4307e25d37991ad331723cdfc6019)) - [@oknozor](https://github.com/oknozor)
- fix release version job output - ([43bb59b](https://github.com/oknozor/toml-bombadil/commit/43bb59b6abe83af010864992b8b22685b66475cd)) - [@oknozor](https://github.com/oknozor)
- use a single release github action workflow - ([46f83aa](https://github.com/oknozor/toml-bombadil/commit/46f83aa7694d3b8135cc19456658bf7c00030ba7)) - [@oknozor](https://github.com/oknozor)
- Fill in missing fmt argument in error message - ([f3cdec1](https://github.com/oknozor/toml-bombadil/commit/f3cdec11b5287d97bf61beb125456e5d93fab6fa)) - David Tolnay
- Secrets are now correctly decryted and injected on install - ([41bdeeb](https://github.com/oknozor/toml-bombadil/commit/41bdeebe32868132873a081be3364c9d85dd786c)) - [@oknozor](https://github.com/oknozor)
- unlink command now correctly remove dots based on previous config - ([caca9aa](https://github.com/oknozor/toml-bombadil/commit/caca9aacc6ee49494cc3c5bbaef01b1eb7fce2c0)) - [@oknozor](https://github.com/oknozor)
- fix empty var files in dot overrides - ([35569cd](https://github.com/oknozor/toml-bombadil/commit/35569cda210d2c15081ba47f9015a119263524a5)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- use nextest for release job - ([210f5ee](https://github.com/oknozor/toml-bombadil/commit/210f5eec2491f9069e10f51511ead9ea4c9dba5f)) - [@oknozor](https://github.com/oknozor)
- ignore conventional commit check for next release - ([a7c8554](https://github.com/oknozor/toml-bombadil/commit/a7c855484e93461884e55f8a74a1705db40ed768)) - [@oknozor](https://github.com/oknozor)
- update release action - ([082bce2](https://github.com/oknozor/toml-bombadil/commit/082bce281c01af1391e72e1b337664d34626d40a)) - [@oknozor](https://github.com/oknozor)
- update action checkout - ([b63f377](https://github.com/oknozor/toml-bombadil/commit/b63f377483d5604c6af34e8329eb268d0edd0bbb)) - [@oknozor](https://github.com/oknozor)
- disable macos - ([9694945](https://github.com/oknozor/toml-bombadil/commit/96949457ba53ea20dd540959e9cd02d940bca5e3)) - [@oknozor](https://github.com/oknozor)
- update rust version in bats container - ([0b0a862](https://github.com/oknozor/toml-bombadil/commit/0b0a8620b4761a07e549591afe3863770a790fe9)) - [@oknozor](https://github.com/oknozor)
- try to fix coverage - ([66531e1](https://github.com/oknozor/toml-bombadil/commit/66531e1c74bb30650d2273ed546b23ed1a5470dc)) - [@oknozor](https://github.com/oknozor)
- add multiarc ci and next test - ([0fd9986](https://github.com/oknozor/toml-bombadil/commit/0fd998691ca4160a10a62d0184c88ffc9ca2536e)) - [@oknozor](https://github.com/oknozor)
- update ci and cog.toml - ([d91c329](https://github.com/oknozor/toml-bombadil/commit/d91c3293660c646d20503e1778cee412cfa03eda)) - [@oknozor](https://github.com/oknozor)
- switch code coverage to cargo-llvm-cov - ([d5db40e](https://github.com/oknozor/toml-bombadil/commit/d5db40e80c077327a0b18a04ea2ba4d640bcf840)) - [@oknozor](https://github.com/oknozor)
- update codecov action - ([9b38b54](https://github.com/oknozor/toml-bombadil/commit/9b38b5433df78f80639944f3bcd5afafe6c2dbea)) - [@oknozor](https://github.com/oknozor)
- add cargo bump to build deps - ([084c8dd](https://github.com/oknozor/toml-bombadil/commit/084c8ddca42438874d4a35d9ace9668256098250)) - [@oknozor](https://github.com/oknozor)
- add automated release, dependabot and github codeowners - ([75ba02e](https://github.com/oknozor/toml-bombadil/commit/75ba02e730efe916a993e054cf61a25d90961007)) - [@oknozor](https://github.com/oknozor)
- fix release action workflow - ([826ad64](https://github.com/oknozor/toml-bombadil/commit/826ad64a15dc487b47cdce9efc9b4e8498d60d24)) - [@oknozor](https://github.com/oknozor)
- add a checkbranch script for cog bump - ([67060df](https://github.com/oknozor/toml-bombadil/commit/67060dfcf6d4844328ba75ff4e636d15dfbec5d6)) - [@oknozor](https://github.com/oknozor)
- prepare website deployment from main branch - ([14be3e2](https://github.com/oknozor/toml-bombadil/commit/14be3e24a1b0dde3f41d87c0d6f4b67dbd60a317)) - [@oknozor](https://github.com/oknozor)
- add github action step for e2e bats tests - ([215f435](https://github.com/oknozor/toml-bombadil/commit/215f435219366f2e76d228e21fe1b8f9c546f29c)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- **(website)** make dark mode default - ([7173844](https://github.com/oknozor/toml-bombadil/commit/7173844ebd1144a9b5a801e24375a8f447d201f0)) - [@oknozor](https://github.com/oknozor)
- **(website)** Update dotfile templating guide for v3 - ([5493b6b](https://github.com/oknozor/toml-bombadil/commit/5493b6ba9d550a7e9afc76dcf8d39d4d22e4d714)) - Arne Beer
- **(website)** Add a changelog blogpost on v3 - ([fbd96ee](https://github.com/oknozor/toml-bombadil/commit/fbd96eeb0c0043fd7b694546bc5a3242b80d88df)) - Arne Beer
- **(website)** fix website logo display - ([0a9da99](https://github.com/oknozor/toml-bombadil/commit/0a9da997bcd92f6ff2e713ab9c33594b00f9472c)) - [@oknozor](https://github.com/oknozor)
- **(website)** update intro and add a changelog blogpost on v2 - ([4e17aa2](https://github.com/oknozor/toml-bombadil/commit/4e17aa25489f7a835525db5584079e75bc6a1223)) - [@oknozor](https://github.com/oknozor)
- **(website)** update profiles and themes sections - ([8bf3319](https://github.com/oknozor/toml-bombadil/commit/8bf331986ed6cbb5d91829becf7950fa59b8af7a)) - [@oknozor](https://github.com/oknozor)
- **(website)** update page on profiles and variables - ([585e764](https://github.com/oknozor/toml-bombadil/commit/585e7644840245e115962180055b0f6393fcf6b3)) - [@oknozor](https://github.com/oknozor)
- add vitepress deployment - ([933b12a](https://github.com/oknozor/toml-bombadil/commit/933b12a1a97cc145ed8db2382e1c678c340aa64c)) - [@oknozor](https://github.com/oknozor)
- migrate to vitepress - ([fccc387](https://github.com/oknozor/toml-bombadil/commit/fccc387f0642a49c5549ec5529fb4683ae9a498b)) - [@oknozor](https://github.com/oknozor)
- remove legacy website - ([f6422f2](https://github.com/oknozor/toml-bombadil/commit/f6422f2edba3a41a0133bb67f9e119c5ce3ab9bb)) - [@oknozor](https://github.com/oknozor)
- add custom hljs theme - ([8c95749](https://github.com/oknozor/toml-bombadil/commit/8c957497eead5e0b191d5b0521e673d3c7f5c898)) - [@oknozor](https://github.com/oknozor)
- migrate to vuepress - ([9de6d96](https://github.com/oknozor/toml-bombadil/commit/9de6d965a952e8006c03203bc1e3936deee3f7be)) - [@oknozor](https://github.com/oknozor)
- remove invalid doc link - ([895f7f8](https://github.com/oknozor/toml-bombadil/commit/895f7f8150987d5053a28411fad705674165bbf3)) - [@oknozor](https://github.com/oknozor)
- add repology badge - ([3ee42aa](https://github.com/oknozor/toml-bombadil/commit/3ee42aa23f24cf273cbae1e1a3f0f81dcee4ca3e)) - [@oknozor](https://github.com/oknozor)
- update doc - ([28cbf71](https://github.com/oknozor/toml-bombadil/commit/28cbf710634603b2758237d656e2fff1c2cb570a)) - [@oknozor](https://github.com/oknozor)
- Fix typos/grammar - ([33a4b47](https://github.com/oknozor/toml-bombadil/commit/33a4b4753b54603b0e5fa597c4627415ec3e7c0b)) - Salim B
- Add updated installation instructions for Arch Linux - ([f8dddf8](https://github.com/oknozor/toml-bombadil/commit/f8dddf8bccf946202483331966fb106c3bdc5538)) - Sven-Hendrik Haase
- update displayed version to be bombadil version - ([69a9c6c](https://github.com/oknozor/toml-bombadil/commit/69a9c6cda14f4aa5882e01b041db3fca62775d4b)) - Thibaud Lepretre
- typo on punctuations - ([4808067](https://github.com/oknozor/toml-bombadil/commit/4808067edb33ade424f9c3c64fd0ecb1f66d26c1)) - Thibaud Lepretre
- remove DSpeckhals example repositories - ([61bf4a5](https://github.com/oknozor/toml-bombadil/commit/61bf4a58dc3c1d3937fbad2bac37cb59c80c274a)) - Thibaud Lepretre
- Link bats HOWTO directly to abs github URL - ([fa6061c](https://github.com/oknozor/toml-bombadil/commit/fa6061c6e0182d054db07b7ac9fcf49f2a312fec)) - Thibaud Lepretre
- update documentation and shorten README - ([928f889](https://github.com/oknozor/toml-bombadil/commit/928f88924e69865c5fc38ac51b6c11ae68ab138e)) - [@oknozor](https://github.com/oknozor)
- fix default language - ([e6e93f0](https://github.com/oknozor/toml-bombadil/commit/e6e93f0d76ad283929751e05c678b2132180d2d0)) - [@oknozor](https://github.com/oknozor)
- document public functions - ([e960f06](https://github.com/oknozor/toml-bombadil/commit/e960f069c53a3fe93bc3bfb653d207d3b5918009)) - [@oknozor](https://github.com/oknozor)
- update link command examples - ([657a1ac](https://github.com/oknozor/toml-bombadil/commit/657a1ac7fabb1afbcff89fe165d7543948d053cd)) - [@oknozor](https://github.com/oknozor)
- Add use-case - ([460ca66](https://github.com/oknozor/toml-bombadil/commit/460ca6642ff7431499521bfe4d64e40ac344a2a4)) - Rohit Goswami
- Fix minor grammar issue - ([caf0d2e](https://github.com/oknozor/toml-bombadil/commit/caf0d2ed0e32ed722046cc55eaf89655162207fc)) - Rohit Goswami
- updated documentation for hooks - ([2146833](https://github.com/oknozor/toml-bombadil/commit/2146833a72b068538506114483bbe2ab925451b6)) - [@travisdavis-ops](https://github.com/travisdavis-ops)
- add link to bombadil website in the readme - ([03d36c1](https://github.com/oknozor/toml-bombadil/commit/03d36c197b923ae16bf1c1bc5795093aa1305655)) - [@oknozor](https://github.com/oknozor)
- update tests and docummentation on profile and vars - ([2da6546](https://github.com/oknozor/toml-bombadil/commit/2da654623dee5f47ce059e4ada01301124ed9caf)) - [@oknozor](https://github.com/oknozor)
- add docs to the website, variable quick start etc - ([0d27e47](https://github.com/oknozor/toml-bombadil/commit/0d27e4748441af293618b3017f3d19d8f52c2e0d)) - [@oknozor](https://github.com/oknozor)
- add a toml bombadil website - ([430c284](https://github.com/oknozor/toml-bombadil/commit/430c284080a46b295c265b1d51abdddc6f7de6b5)) - [@oknozor](https://github.com/oknozor)
- mrkajetanp's dotfiles in example repositories - ([8becff4](https://github.com/oknozor/toml-bombadil/commit/8becff48b6b99c13a72ab8be775d9b3f9864c798)) - Kajetan Puchalski
#### Features
- **(intall)** add preinstall configuration clean up - ([16c74cf](https://github.com/oknozor/toml-bombadil/commit/16c74cfc20e72f2cf2029bcaa4e0dce434885fdc)) - [@oknozor](https://github.com/oknozor)
- **(link)** link only dotfiles with diff - ([4f0be1f](https://github.com/oknozor/toml-bombadil/commit/4f0be1fd7c2d97bf58e4874bf877a824ea4bef92)) - [@oknozor](https://github.com/oknozor)
- **(template)** access profiles in tera context - ([47f69b0](https://github.com/oknozor/toml-bombadil/commit/47f69b05129357997b184c3ddac87c2b253bdd12)) - [@oknozor](https://github.com/oknozor)
- **(templates)** rework template system - ([3127671](https://github.com/oknozor/toml-bombadil/commit/3127671acd27c52b387a5ca67bb5fdde302c8d26)) - [@oknozor](https://github.com/oknozor)
- add OS template var - ([8026e2c](https://github.com/oknozor/toml-bombadil/commit/8026e2c47cee5d8fa28a106475bc52d36365f130)) - [@oknozor](https://github.com/oknozor)
- Add bombadil watch feature - ([09f1a99](https://github.com/oknozor/toml-bombadil/commit/09f1a99cdb740a3e1c760769083e212c32dea409)) - Sven-Hendrik Haase
- use tera for templating - ([4a55b06](https://github.com/oknozor/toml-bombadil/commit/4a55b0603289554113a723716f6c072135650529)) - Arne Beer
- add extra profiles - ([fb947b5](https://github.com/oknozor/toml-bombadil/commit/fb947b53be32992a8ff18f2e9b5e6852eb8c47db)) - [@oknozor](https://github.com/oknozor)
- clone dotfiles from remote - ([fa0964a](https://github.com/oknozor/toml-bombadil/commit/fa0964ad65b64bb186dc14a9d13883c38389cbd1)) - [@oknozor](https://github.com/oknozor)
- add pre&post hook to get command - ([a6bf4f2](https://github.com/oknozor/toml-bombadil/commit/a6bf4f2ce70650c58a07561148b546e33d69e713)) - [@travisdavis-ops](https://github.com/travisdavis-ops)
- add post install hooks, rename hooks to posthooks - ([f5dcc34](https://github.com/oknozor/toml-bombadil/commit/f5dcc34647664edb3d5f0c4dd35f524f344eb8fb)) - [@travisdavis-ops](https://github.com/travisdavis-ops)
#### Miscellaneous Chores
- **(deps)** bump speculoos from 0.9.0 to 0.10.0 - ([77e71b9](https://github.com/oknozor/toml-bombadil/commit/77e71b961f9b6cf28d8a493846b8a1d399685696)) - dependabot[bot]
- **(deps)** bump anyhow from 1.0.53 to 1.0.55 - ([af36b76](https://github.com/oknozor/toml-bombadil/commit/af36b76ae084d242081928344832c6f9614441f1)) - dependabot[bot]
- **(deps)** update dirs requirement from ^3 to ^4 - ([c2ad1ce](https://github.com/oknozor/toml-bombadil/commit/c2ad1ce3becd7f49caa094bc282655d306189db7)) - dependabot[bot]
- **(version)** 3.1.0 - ([92382b3](https://github.com/oknozor/toml-bombadil/commit/92382b316b9e6e2c61ed40dd94eb43012d154ffa)) - cog-bot
- **(version)** 3.0.0 - ([c9cd761](https://github.com/oknozor/toml-bombadil/commit/c9cd761bc689d1132ffdabef10a07e615d7a9058)) - cog-bot
- **(version)** 2.2.4 - ([25ef69d](https://github.com/oknozor/toml-bombadil/commit/25ef69d96d9b34c539f017a134f4e4721c3a5423)) - cog-bot
- **(version)** 2.2.3 - ([9f63058](https://github.com/oknozor/toml-bombadil/commit/9f630589bbb336295e681b7ca1105c3d63d521ba)) - cog-bot
- **(version)** 2.2.2 - ([6c21406](https://github.com/oknozor/toml-bombadil/commit/6c21406cf5b3f86bb0d9006921ba97eeffa032e0)) - cog-bot
- **(version)** 2.2.1 - ([d8224db](https://github.com/oknozor/toml-bombadil/commit/d8224db6a4f814fb40c6b96485697a98855422d9)) - cog-bot
- **(version)** 2.2.0 - ([5470287](https://github.com/oknozor/toml-bombadil/commit/54702871a88bebb00fa1f37f8ab3a7290842173b)) - cog-bot
- **(version)** 2.1.0 - ([80219a2](https://github.com/oknozor/toml-bombadil/commit/80219a2885afa136822fb8b4d4c1058f7189d001)) - [@oknozor](https://github.com/oknozor)
- **(version)** 2.0.0 - ([876646f](https://github.com/oknozor/toml-bombadil/commit/876646f28a751ab0775660f46f7f7bc0823d708e)) - [@oknozor](https://github.com/oknozor)
- add cargo package exclusion - ([1b56151](https://github.com/oknozor/toml-bombadil/commit/1b56151eea7d833737457df19365e4d49288bf8c)) - [@oknozor](https://github.com/oknozor)
- update rust edition and release profile - ([d9c860b](https://github.com/oknozor/toml-bombadil/commit/d9c860b015d431ce01084dbbc81209d295c5a77d)) - [@oknozor](https://github.com/oknozor)
- update dependencies - ([87489d0](https://github.com/oknozor/toml-bombadil/commit/87489d0db21243ab88bb2b6fee67464f22022b02)) - [@oknozor](https://github.com/oknozor)
- cargo update - ([595f868](https://github.com/oknozor/toml-bombadil/commit/595f8680e29e7b9052b1540650fd0936820df6bb)) - [@oknozor](https://github.com/oknozor)
- fix gh pages base path - ([77bd540](https://github.com/oknozor/toml-bombadil/commit/77bd54002d8c28ed540b06ac70717d8d1528d647)) - [@oknozor](https://github.com/oknozor)
- fix gh token permission - ([11a8592](https://github.com/oknozor/toml-bombadil/commit/11a85927a8531c445f029a3ea5e0d4b4251884ea)) - [@oknozor](https://github.com/oknozor)
- fix node setup - ([b3cb614](https://github.com/oknozor/toml-bombadil/commit/b3cb614103c2f4db88ad52828cd6d0dfb4f1a05f)) - [@oknozor](https://github.com/oknozor)
- fix typo - ([c0eb45f](https://github.com/oknozor/toml-bombadil/commit/c0eb45f682a1b032c54c2e3bb0483f28a273c7c7)) - [@oknozor](https://github.com/oknozor)
- update deps - ([855606d](https://github.com/oknozor/toml-bombadil/commit/855606db48b51f5850ebdeaf18ed52bdfcf03ac6)) - [@oknozor](https://github.com/oknozor)
- update git attributes - ([ec427ba](https://github.com/oknozor/toml-bombadil/commit/ec427ba55ad15a7693fba887c744b03a2411032f)) - [@oknozor](https://github.com/oknozor)
- fmt all - ([2c31707](https://github.com/oknozor/toml-bombadil/commit/2c3170730a2501dcb5abaec941e03f28d55e35b7)) - [@oknozor](https://github.com/oknozor)
- gitignore static assets - ([50e525f](https://github.com/oknozor/toml-bombadil/commit/50e525ff3c088fa697ea2c524d846df97f8a40ff)) - [@oknozor](https://github.com/oknozor)
- update deps and fix clippy lints - ([008fd3b](https://github.com/oknozor/toml-bombadil/commit/008fd3b7015a8afe0be333896fde6196138d1763)) - Dominik Nakamura
- Minor typo - ([9ae4f9e](https://github.com/oknozor/toml-bombadil/commit/9ae4f9e0348954bc9d91d86f7ff410fea3744598)) - Shoyu Vanilla
- remove website build before migrating to vuepress - ([d5b4ce3](https://github.com/oknozor/toml-bombadil/commit/d5b4ce3c8e5b853c35b77897666bfc0dfa5984b1)) - [@oknozor](https://github.com/oknozor)
- ignore website with github linguist - ([604e83d](https://github.com/oknozor/toml-bombadil/commit/604e83df51138c5636924931330221725f86d853)) - [@oknozor](https://github.com/oknozor)
- bump clap to v4 - ([e70f7cf](https://github.com/oknozor/toml-bombadil/commit/e70f7cf68c572570b647110b8b479e9df888f23c)) - [@oknozor](https://github.com/oknozor)
- bump cargo deps - ([7984530](https://github.com/oknozor/toml-bombadil/commit/7984530cc5f678e15feb4b1417ec67f3ea8aab34)) - [@oknozor](https://github.com/oknozor)
- clippy lints - ([d42dff2](https://github.com/oknozor/toml-bombadil/commit/d42dff2b080f2bdafdcb3834a67eb08576c1ed81)) - [@oknozor](https://github.com/oknozor)
- bump dependencies - ([9ecbc7f](https://github.com/oknozor/toml-bombadil/commit/9ecbc7f0726d60511c38f812ea0e878fdba25546)) - [@oknozor](https://github.com/oknozor)
- remove install module - ([5f35e6d](https://github.com/oknozor/toml-bombadil/commit/5f35e6d1ea7d372c6c9061cde13f39f376297401)) - [@oknozor](https://github.com/oknozor)
- use a fixed version of rust in bats test docker image - ([d3562e6](https://github.com/oknozor/toml-bombadil/commit/d3562e6b0b0be4170b0830cc3cf67818ab718ce6)) - [@oknozor](https://github.com/oknozor)
- bump libgit2 to 0.14 - ([77de6aa](https://github.com/oknozor/toml-bombadil/commit/77de6aa3d220b957a81e425e0a8a3f79108d19e6)) - [@oknozor](https://github.com/oknozor)
- Various smaller doc fixes - ([30aaf76](https://github.com/oknozor/toml-bombadil/commit/30aaf7687d15e0cb43509269cba21719b6829e67)) - Sven-Hendrik Haase
- Fix Arch installation instructions in book - ([02c3580](https://github.com/oknozor/toml-bombadil/commit/02c3580228532ef828a9da2006a5f646ebd017cb)) - Sven-Hendrik Haase
- Add codecov.yml and configure a threshold of 1% - ([70d1820](https://github.com/oknozor/toml-bombadil/commit/70d182095a5c56897d24e3ad35ce61a0ce93384b)) - Sven-Hendrik Haase
- use 2018 module imports - ([62e0b6b](https://github.com/oknozor/toml-bombadil/commit/62e0b6b5cf09538a608075d4a84d0914f2f8fec2)) - Arne Beer
- Add cargo.lock to project - ([4e27d48](https://github.com/oknozor/toml-bombadil/commit/4e27d486624e88cf7636c9a6e392b45ef59e8681)) - Arne Beer
- fix clippy lints - ([fe6d1ca](https://github.com/oknozor/toml-bombadil/commit/fe6d1caf869ea7e200731a8da135f8c1c5313daa)) - [@oknozor](https://github.com/oknozor)
- update cocogitto bump config - ([a869b56](https://github.com/oknozor/toml-bombadil/commit/a869b568b12052f6d05c21c851a9c4d3a9df9045)) - [@oknozor](https://github.com/oknozor)
- add github sponsor - ([0ee2fad](https://github.com/oknozor/toml-bombadil/commit/0ee2fad28e914383abd0f10c0a3192a5bac11a1c)) - [@oknozor](https://github.com/oknozor)
- clippy lints and fmt * - ([5d35aea](https://github.com/oknozor/toml-bombadil/commit/5d35aea53eacdc35831b0dd94f2e5723b7dc39d6)) - [@oknozor](https://github.com/oknozor)
- fix action CD script - ([751607f](https://github.com/oknozor/toml-bombadil/commit/751607f9ed6d0fd2b06bbda6684b0656a75b2965)) - [@oknozor](https://github.com/oknozor)
- ignore aur package - ([c977926](https://github.com/oknozor/toml-bombadil/commit/c9779269eb157dbeabe486ef4f855d872e859451)) - [@oknozor](https://github.com/oknozor)
- remove aur submodule, it can't be checked out in github CI - ([4f9dca0](https://github.com/oknozor/toml-bombadil/commit/4f9dca03b1bb7d70d929f91704fcf5a3b3c92efb)) - [@oknozor](https://github.com/oknozor)
- add cargo home page and cog bump config - ([4a7756c](https://github.com/oknozor/toml-bombadil/commit/4a7756c8a1ce4b0e6c3d28ed49fba27a272df6ad)) - [@oknozor](https://github.com/oknozor)
- add aur package as a submodule - ([1aee7b6](https://github.com/oknozor/toml-bombadil/commit/1aee7b679bcf2ed5dd49d4961ac854fb03262868)) - [@oknozor](https://github.com/oknozor)
- deploy GH page with github actions - ([a782aa1](https://github.com/oknozor/toml-bombadil/commit/a782aa143e00e24278323e0ad562fd9fe19727d3)) - [@oknozor](https://github.com/oknozor)
- fix bats-file submodule remote - ([1ea9520](https://github.com/oknozor/toml-bombadil/commit/1ea9520b6f02b46cec87c17ad1671bb2384627bf)) - [@oknozor](https://github.com/oknozor)
- fix clippy lints - ([6c919fa](https://github.com/oknozor/toml-bombadil/commit/6c919fad9cf1859c200d4bb1eabf38759f1b3715)) - [@oknozor](https://github.com/oknozor)
#### Refactoring
- **(cli)** update to clap v3 - ([c7bc781](https://github.com/oknozor/toml-bombadil/commit/c7bc7817ad3323a47d9bfadbaa2c58cca4255999)) - [@oknozor](https://github.com/oknozor)
- make profile template context global - ([1840db7](https://github.com/oknozor/toml-bombadil/commit/1840db7949d605ffc1e74dc9238fa93f1b257900)) - [@oknozor](https://github.com/oknozor)
- improve link display on watch/link - ([f9ea3f9](https://github.com/oknozor/toml-bombadil/commit/f9ea3f9adb7dae8b25686868516f85c812111c0f)) - [@oknozor](https://github.com/oknozor)
- extract display logic to a decicated module - ([d87e6ed](https://github.com/oknozor/toml-bombadil/commit/d87e6ed0d9bc572621b0bc3ed8cb8357e1572d3e)) - [@oknozor](https://github.com/oknozor)
- use lazy static for gpg and dotfile dir - ([9aa911c](https://github.com/oknozor/toml-bombadil/commit/9aa911cdc98ccaaaebc18d0c0982842ef91106aa)) - [@oknozor](https://github.com/oknozor)
- split paths and settings into dedicated modules - ([13d05dc](https://github.com/oknozor/toml-bombadil/commit/13d05dc55fc56f419b297fd7d704f58feecc441a)) - [@oknozor](https://github.com/oknozor)
- check profile activation in the lib instead of the cli - ([b739828](https://github.com/oknozor/toml-bombadil/commit/b739828121df616580f5d963c14f5cf4c37e95ab)) - [@oknozor](https://github.com/oknozor)
- simplify bombadil link - ([db89f37](https://github.com/oknozor/toml-bombadil/commit/db89f37bfd4840c38d18b53029182d0a942b7436)) - [@oknozor](https://github.com/oknozor)
#### Tests
- **(bats)** add tilde shortand to bashrc home - ([e8d9db3](https://github.com/oknozor/toml-bombadil/commit/e8d9db3c80522728e81e48b683bb15aa8e57edd4)) - [@oknozor](https://github.com/oknozor)
- **(git)** add bombadil clone test - ([84025d8](https://github.com/oknozor/toml-bombadil/commit/84025d8e1c754d468e80eca6fabd15d0ff0444a9)) - [@oknozor](https://github.com/oknozor)
- fix tests on macos - ([c568aa3](https://github.com/oknozor/toml-bombadil/commit/c568aa33642d4bc233619a918fb9e531cd5ff243)) - [@oknozor](https://github.com/oknozor)
- try to fix gpg - ([e492cef](https://github.com/oknozor/toml-bombadil/commit/e492cef1796d260cc1ede294fd5e64b068804b21)) - [@oknozor](https://github.com/oknozor)
- try to ignore macos for symlink tests - ([e2fe751](https://github.com/oknozor/toml-bombadil/commit/e2fe75180db635f8734b0a235a667e8d7b1b5ab6)) - [@oknozor](https://github.com/oknozor)
- add debug print - ([4ff1ceb](https://github.com/oknozor/toml-bombadil/commit/4ff1ceb9a158ed0d6dd52a12fb850291f5bdbf0e)) - [@oknozor](https://github.com/oknozor)
- fix macos test - ([b6dc213](https://github.com/oknozor/toml-bombadil/commit/b6dc213ee8575c95a1749cc1bafe6ac53aa840e6)) - [@oknozor](https://github.com/oknozor)
- try on one thread - ([7d25d5e](https://github.com/oknozor/toml-bombadil/commit/7d25d5e670dd5728efd6e49c628bdeb4184d63b2)) - [@oknozor](https://github.com/oknozor)
- update rust docker version - ([0c0aeb1](https://github.com/oknozor/toml-bombadil/commit/0c0aeb10d465a4d5cffc1ea33cd8d82c6bf95616)) - [@oknozor](https://github.com/oknozor)
- fix tests - ([a439bd9](https://github.com/oknozor/toml-bombadil/commit/a439bd96436ca3528aa10a4b652593bc27083767)) - [@oknozor](https://github.com/oknozor)
- fix unlink test on CI - ([8c06b6d](https://github.com/oknozor/toml-bombadil/commit/8c06b6da2900f17eff1d4448523651e8da4ff9f2)) - [@oknozor](https://github.com/oknozor)
- fix vendored openssl for bats tests - ([93ed4e0](https://github.com/oknozor/toml-bombadil/commit/93ed4e08028d47245270cea5965c5161f76d8e9c)) - [@oknozor](https://github.com/oknozor)
- add test for metadata print - ([af3a054](https://github.com/oknozor/toml-bombadil/commit/af3a0540c5e7d1d32908510a3dfa7c4507b88a81)) - [@oknozor](https://github.com/oknozor)
- change expired gpg test keys - ([87b7fbf](https://github.com/oknozor/toml-bombadil/commit/87b7fbff23f701ec6b4cbb0b03c2fb85a5eef4af)) - [@oknozor](https://github.com/oknozor)
- fix bats test and add dockerignore - ([69bfa9f](https://github.com/oknozor/toml-bombadil/commit/69bfa9fb88aa738631b97d40d1361c03979e5495)) - [@oknozor](https://github.com/oknozor)
- add sealed test to preserve developer environement - ([485df1f](https://github.com/oknozor/toml-bombadil/commit/485df1f387860818aedd21d7eae89c8cb0c941b3)) - [@oknozor](https://github.com/oknozor)
- fix config path in tests - ([4d047de](https://github.com/oknozor/toml-bombadil/commit/4d047de9dd6238531e7ad22005abae7f21dd586e)) - [@oknozor](https://github.com/oknozor)
- fixed test code - ([cd61a66](https://github.com/oknozor/toml-bombadil/commit/cd61a66baf87e34cf4cb7c033b8f03c0ede6d323)) - [@travisdavis-ops](https://github.com/travisdavis-ops)
- add dockerized bats test suite - ([fc0e33d](https://github.com/oknozor/toml-bombadil/commit/fc0e33da0d6305d0f68adebb7ee0753b967a4ac0)) - [@oknozor](https://github.com/oknozor)

- - -

## [3.1.0](https://github.com/oknozor/toml-bombadil/compare/3.0.0..3.1.0) - 2022-05-11
#### Bug Fixes
- display undeclared variables when rendering templates - ([5e3c148](https://github.com/oknozor/toml-bombadil/commit/5e3c1481e2549ebc063432df99340d2e0021f84a)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- switch code coverage to cargo-llvm-cov - ([7e31e00](https://github.com/oknozor/toml-bombadil/commit/7e31e0046885c67c9810936d6e0bcfeb5974cdf4)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- **(website)** Update dotfile templating guide for v3 - ([90b43e8](https://github.com/oknozor/toml-bombadil/commit/90b43e8c1d20c890922fd41a81d79e00c1f9ba0a)) - Arne Beer
- **(website)** Add a changelog blogpost on v3 - ([f789e22](https://github.com/oknozor/toml-bombadil/commit/f789e22286f1d1593e0df36c071a559780326124)) - Arne Beer
- Add updated installation instructions for Arch Linux - ([900a994](https://github.com/oknozor/toml-bombadil/commit/900a99485fe6930f5a76b79666ae8b74010cd3b8)) - Sven-Hendrik Haase
#### Features
- Add bombadil watch feature - ([e5b8ceb](https://github.com/oknozor/toml-bombadil/commit/e5b8ceb5febbdcee1250218fc9159f4429c4106d)) - Sven-Hendrik Haase
#### Miscellaneous Chores
- use a fixed version of rust in bats test docker image - ([9e11fe7](https://github.com/oknozor/toml-bombadil/commit/9e11fe7a755837d87182bc0877e06084200e8b02)) - [@oknozor](https://github.com/oknozor)
- bump libgit2 to 0.14 - ([4c6009e](https://github.com/oknozor/toml-bombadil/commit/4c6009eeade1d2c89c7f3a9083149f065601998d)) - [@oknozor](https://github.com/oknozor)
- Various smaller doc fixes - ([9b03122](https://github.com/oknozor/toml-bombadil/commit/9b031226eb9320569d1a16feeb141cb15fe2eb86)) - Sven-Hendrik Haase
- Fix Arch installation instructions in book - ([3ecc202](https://github.com/oknozor/toml-bombadil/commit/3ecc202d6c1de92724b527d6cdaa977e0cfe4a9f)) - Sven-Hendrik Haase
#### Refactoring
- **(cli)** update to clap v3 - ([c3e798c](https://github.com/oknozor/toml-bombadil/commit/c3e798c122d5f418eaae519629587f2600e56a71)) - [@oknozor](https://github.com/oknozor)
- check profile activation in the lib instead of the cli - ([eb025ad](https://github.com/oknozor/toml-bombadil/commit/eb025ad130238a80accda38c326a492df9d4aa24)) - [@oknozor](https://github.com/oknozor)
- simplify bombadil link - ([7076663](https://github.com/oknozor/toml-bombadil/commit/70766631276511ba57f2e0623be6a17fb6ea1fd6)) - [@oknozor](https://github.com/oknozor)
#### Tests
- add sealed test to preserve developer environement - ([455eef3](https://github.com/oknozor/toml-bombadil/commit/455eef39b8df58fb9e7eabba98dcbfac93220498)) - [@oknozor](https://github.com/oknozor)
- - -

## [3.0.0](https://github.com/oknozor/toml-bombadil/compare/2.2.4..3.0.0) - 2022-02-23
#### Features
- use tera for templating - ([0a528fd](https://github.com/oknozor/toml-bombadil/commit/0a528fdf0cc284726ab0c25afc7702ebe8a2fcc5)) - Arne Beer
#### Miscellaneous Chores
- **(deps)** bump anyhow from 1.0.53 to 1.0.55 - ([787b4ba](https://github.com/oknozor/toml-bombadil/commit/787b4ba8e1993a78eddda77fc4b4bde5165047f8)) - dependabot[bot]
- Add codecov.yml and configure a threshold of 1% - ([c57a5d5](https://github.com/oknozor/toml-bombadil/commit/c57a5d567be68c5ef35d209c1c7a546031f16686)) - Sven-Hendrik Haase
- - -

## [2.2.4](https://github.com/oknozor/toml-bombadil/compare/2.2.3..2.2.4) - 2022-02-19
#### Bug Fixes
- run cargo bump early in cog.toml so Cargo.lock is updated after the release build - ([3880658](https://github.com/oknozor/toml-bombadil/commit/38806587c312d08a5e90269e351264d1312606bb)) - [@oknozor](https://github.com/oknozor)
- fix release github action workflow - ([eacf932](https://github.com/oknozor/toml-bombadil/commit/eacf932459051eb87b083dc0077d1419ee91cb9e)) - [@oknozor](https://github.com/oknozor)
#### Continuous Integration
- update codecov action - ([87eeeb5](https://github.com/oknozor/toml-bombadil/commit/87eeeb521910822f10c5bbee56b11ae7abe69625)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- update displayed version to be bombadil version - ([409baf0](https://github.com/oknozor/toml-bombadil/commit/409baf0cdb803dd8305f95a77954534ef5cda88e)) - Thibaud Lepretre
- typo on punctuations - ([f6dae99](https://github.com/oknozor/toml-bombadil/commit/f6dae99abce5422540b7317dcee882c44bbaaaed)) - Thibaud Lepretre
- remove DSpeckhals example repositories - ([d516414](https://github.com/oknozor/toml-bombadil/commit/d5164148d6b8c6aace16e1849f494ee73c98eafa)) - Thibaud Lepretre
- Link bats HOWTO directly to abs github URL - ([d4d87a8](https://github.com/oknozor/toml-bombadil/commit/d4d87a85b591e63489ae5f99810a8fd3ad05015e)) - Thibaud Lepretre
#### Miscellaneous Chores
- use 2018 module imports - ([ae610e0](https://github.com/oknozor/toml-bombadil/commit/ae610e01d3c341989c39a0b2a7b50404706d4301)) - Arne Beer
- Add cargo.lock to project - ([bac3b31](https://github.com/oknozor/toml-bombadil/commit/bac3b316e8ea5a58e6d5d88894d9da1ddd29a7f9)) - Arne Beer
- - -

## [2.2.3](https://github.com/oknozor/toml-bombadil/compare/2.2.2..2.2.3) - 2021-12-30
#### Bug Fixes
- fix release version job output again - ([a9963e7](https://github.com/oknozor/toml-bombadil/commit/a9963e7d6f3bcb5f287e0d60b801d6defedf5732)) - [@oknozor](https://github.com/oknozor)
- - -

## [2.2.2](https://github.com/oknozor/toml-bombadil/compare/2.2.1..2.2.2) - 2021-12-30
#### Bug Fixes
- fix release version job output - ([0279e96](https://github.com/oknozor/toml-bombadil/commit/0279e96788bffc4e6a669ecc7e7538361db0b108)) - [@oknozor](https://github.com/oknozor)
- - -

## [2.2.1](https://github.com/oknozor/toml-bombadil/compare/2.2.0..2.2.1) - 2021-12-30
#### Bug Fixes
- use a single release github action workflow - ([e104eb0](https://github.com/oknozor/toml-bombadil/commit/e104eb023578bfd2aff7bb20db7b6f1e11d64f2a)) - [@oknozor](https://github.com/oknozor)
- - -

## [2.2.0](https://github.com/oknozor/toml-bombadil/compare/2.1.0..2.2.0) - 2021-12-30
#### Continuous Integration
- add cargo bump to build deps - ([a321566](https://github.com/oknozor/toml-bombadil/commit/a321566a4608f582fc8dc10c75c199f84a8fe914)) - [@oknozor](https://github.com/oknozor)
- add automated release, dependabot and github codeowners - ([9307457](https://github.com/oknozor/toml-bombadil/commit/93074570c2d70a501ba136cb645b53b13e284c13)) - [@oknozor](https://github.com/oknozor)
#### Documentation
- update documentation and shorten README - ([3780d09](https://github.com/oknozor/toml-bombadil/commit/3780d0984eaec42c7c64229e02810d5b94a58080)) - [@oknozor](https://github.com/oknozor)
#### Features
- add extra profiles - ([7741f39](https://github.com/oknozor/toml-bombadil/commit/7741f399069da6c48f58a44e102b73270e831170)) - [@oknozor](https://github.com/oknozor)
#### Miscellaneous Chores
- **(deps)** update dirs requirement from ^3 to ^4 - ([2ee87e0](https://github.com/oknozor/toml-bombadil/commit/2ee87e0d19d90f1adc17628aaf751056a08b71b5)) - dependabot[bot]
- fix clippy lints - ([482e9ae](https://github.com/oknozor/toml-bombadil/commit/482e9ae0408b6164a355f5c3f41288e7e43aeff9)) - [@oknozor](https://github.com/oknozor)
- - -

## [2.1.0](https://github.com/oknozor/toml-bombadil/compare/2.0.0..2.1.0) - 2021-11-21
#### Bug Fixes
- Fill in missing fmt argument in error message - ([14ed188](https://github.com/oknozor/toml-bombadil/commit/14ed18862e826e46642bdeb725d42a69da01e17a)) - David Tolnay
#### Documentation
- **(website)** fix website logo display - ([da2efdf](https://github.com/oknozor/toml-bombadil/commit/da2efdf140dfdd2f285dcbc57403d994a4f2800e)) - [@oknozor](https://github.com/oknozor)
- fix default language - ([9a43dca](https://github.com/oknozor/toml-bombadil/commit/9a43dca38467e053a6bdf447f3dd709e4d8b4dbe)) - [@oknozor](https://github.com/oknozor)
- document public functions - ([ede6828](https://github.com/oknozor/toml-bombadil/commit/ede68283e56b84244b46d730d83a948272da755a)) - [@oknozor](https://github.com/oknozor)
- update link command examples - ([8485455](https://github.com/oknozor/toml-bombadil/commit/8485455052c31534b442e974387ddaf9f731fa2e)) - [@oknozor](https://github.com/oknozor)
- Add use-case - ([c5e3c81](https://github.com/oknozor/toml-bombadil/commit/c5e3c8106c4e44c84d6209ff5984ddd3b603885f)) - Rohit Goswami
- Fix minor grammar issue - ([9f56272](https://github.com/oknozor/toml-bombadil/commit/9f562723ca0d1dce247f175756f8a04c5a8f7459)) - Rohit Goswami
- updated documentation for hooks - ([da56a4d](https://github.com/oknozor/toml-bombadil/commit/da56a4d481d2d78bdaa0fcd5c49239c5073feb37)) - [@travisdavis-ops](https://github.com/travisdavis-ops)
- add link to bombadil website in the readme - ([37096cd](https://github.com/oknozor/toml-bombadil/commit/37096cd03cd18dbefdff4f2e837a711bbc00b3ca)) - [@oknozor](https://github.com/oknozor)
#### Features
- clone dotfiles from remote - ([98afb6c](https://github.com/oknozor/toml-bombadil/commit/98afb6cb18b7c268e5ac64dcb00f567615aaf1f0)) - [@oknozor](https://github.com/oknozor)
- add pre&post hook to get command - ([2e7b651](https://github.com/oknozor/toml-bombadil/commit/2e7b651a851c6548d686ba393a17f0309827a585)) - [@travisdavis-ops](https://github.com/travisdavis-ops)
- add post install hooks, rename hooks to posthooks - ([4a21843](https://github.com/oknozor/toml-bombadil/commit/4a21843b4396e368120c90ee031c8e7ba7f3ce6c)) - [@travisdavis-ops](https://github.com/travisdavis-ops)
#### Miscellaneous Chores
- update cocogitto bump config - ([078d22b](https://github.com/oknozor/toml-bombadil/commit/078d22b290ba93a681156789107718222b326021)) - [@oknozor](https://github.com/oknozor)
- add github sponsor - ([20696fc](https://github.com/oknozor/toml-bombadil/commit/20696fc13ba33f0ad74930102081ce9113a32261)) - [@oknozor](https://github.com/oknozor)
- clippy lints and fmt * - ([36b82b0](https://github.com/oknozor/toml-bombadil/commit/36b82b02fa07bf0f908983906c3d95b2ddf377af)) - [@oknozor](https://github.com/oknozor)
#### Tests
- fix config path in tests - ([3032204](https://github.com/oknozor/toml-bombadil/commit/3032204e607c7abee71383f722455b4fca8752c2)) - [@oknozor](https://github.com/oknozor)
- fixed test code - ([337874e](https://github.com/oknozor/toml-bombadil/commit/337874e12cb386f484f4897e09e6d320b0fbc270)) - [@travisdavis-ops](https://github.com/travisdavis-ops)
- - -

## 2.0.0 - 2021-05-25


### Documentation

[0f6b10](https://github.com/oknozor/toml-bombadil/commit/0f6b104d0f55bec7a3ad46e41e9e8bb314085f74) - update intro and add a changelog blogpost on v2 - [oknozor](https://github.com/oknozor)

[becad6](https://github.com/oknozor/toml-bombadil/commit/becad6db4fd698665cb5797206758c467ed23e82) - update profiles and themes sections - [oknozor](https://github.com/oknozor)

[edbe4b](https://github.com/oknozor/toml-bombadil/commit/edbe4bff76e0e7dd9502a8df8ec6229858470f07) - update tests and docummentation on profile and vars - [oknozor](https://github.com/oknozor)

[b65d15](https://github.com/oknozor/toml-bombadil/commit/b65d150a1ab158be101f8cd8ec52754c92968a69) - update page on profiles and variables - [oknozor](https://github.com/oknozor)

[995ff2](https://github.com/oknozor/toml-bombadil/commit/995ff2d3e96d77d0ad8ac13c0ac0d9ab6ee92645) - add docs to the website, variable quick start etc - [oknozor](https://github.com/oknozor)

[48b74b](https://github.com/oknozor/toml-bombadil/commit/48b74b7cbeba11fe024c6e4a97a1601cc1996bb0) - add a toml bombadil website - [oknozor](https://github.com/oknozor)

[cf243e](https://github.com/oknozor/toml-bombadil/commit/cf243e94197e59e49b4ede630be8efba2f641ab5) - add minor corrections to contributing guidelines - [oknozor](https://github.com/oknozor)

[872e7f](https://github.com/oknozor/toml-bombadil/commit/872e7f3487f93bc0edf3f83cdaea7973add0e11f) - add small corrections to the readme - [oknozor](https://github.com/oknozor)

[aff785](https://github.com/oknozor/toml-bombadil/commit/aff785ca0faa75a117e9bc65d82cc68df66f653d) - update readme - [oknozor](https://github.com/oknozor)

[4f392a](https://github.com/oknozor/toml-bombadil/commit/4f392af305f98d7d0fee0077e7c7a92bb6d9d7d7) - add readme instruction for secret vars - [oknozor](https://github.com/oknozor)


### Continuous Integration

[e1b25d](https://github.com/oknozor/toml-bombadil/commit/e1b25d15df7628a21325d59c331304a24b2f53fa) - add a checkbranch script for cog bump - [oknozor](https://github.com/oknozor)

[f479e1](https://github.com/oknozor/toml-bombadil/commit/f479e1337bcfb452033e13427a536f7e79b100c0) - prepare website deployment from main branch - [oknozor](https://github.com/oknozor)

[7ef913](https://github.com/oknozor/toml-bombadil/commit/7ef913eb7f1d496409421291cb8f01c187b95aa9) - add github action step for e2e bats tests - [oknozor](https://github.com/oknozor)


### Miscellaneous Chores

[2717e8](https://github.com/oknozor/toml-bombadil/commit/2717e86f1e21dbcd5e7c595fa53f56b57cb80295) - fix action CD script - [oknozor](https://github.com/oknozor)

[ea6eac](https://github.com/oknozor/toml-bombadil/commit/ea6eac452bb5d78959e8a4fc485be197827a4578) - ignore aur package - [oknozor](https://github.com/oknozor)

[846550](https://github.com/oknozor/toml-bombadil/commit/8465509c62b82d507800cb5e93fe6798fdc9025f) - remove aur submodule, it can't be checked out in github CI - [oknozor](https://github.com/oknozor)

[8c1cf8](https://github.com/oknozor/toml-bombadil/commit/8c1cf88b4fce2556f09899a1596e3eeb82e8bb04) - add cargo home page and cog bump config - [oknozor](https://github.com/oknozor)

[1034c3](https://github.com/oknozor/toml-bombadil/commit/1034c36e11ca0dcc78d5b2d3d87eb433ff877bcb) - add aur package as a submodule - [oknozor](https://github.com/oknozor)

[37344d](https://github.com/oknozor/toml-bombadil/commit/37344d1c163d35bfecf66124e78a0602b7d1b91d) - deploy GH page with github actions - [oknozor](https://github.com/oknozor)

[8035c6](https://github.com/oknozor/toml-bombadil/commit/8035c61ef5e2dfeab77995325fae3f55478cf0d5) - fix bats-file submodule remote - [oknozor](https://github.com/oknozor)

[f5a326](https://github.com/oknozor/toml-bombadil/commit/f5a326a5d79e0a19a1344c58d3436f04720a5167) - fix clippy lints - [oknozor](https://github.com/oknozor)

[a69ab2](https://github.com/oknozor/toml-bombadil/commit/a69ab255c8a287a4f7e6aadb3fefdf0893d4fee1) - fmt all - [oknozor](https://github.com/oknozor)

[443e2e](https://github.com/oknozor/toml-bombadil/commit/443e2e6b3ab4e5fd4bcb9a19e971b94be0a6a036) - add contributors and bump hook to cog config - [oknozor](https://github.com/oknozor)

[99176d](https://github.com/oknozor/toml-bombadil/commit/99176dd79b19cefd80e973213f424def7c3dd994) - fix clippy lints - [oknozor](https://github.com/oknozor)

[6608b9](https://github.com/oknozor/toml-bombadil/commit/6608b9c4924ac351d91efb63435311d826565cf5) - fmt all - [oknozor](https://github.com/oknozor)

[4d7fc7](https://github.com/oknozor/toml-bombadil/commit/4d7fc733f0cc1074ab8c0a4465249a5366cbb7be) - Address review comments - [DSpeckhals](https://github.com/DSpeckhals)

[cd748d](https://github.com/oknozor/toml-bombadil/commit/cd748d21a294bb598f857b022455963aeb91a6fe) - bump crates.io version - [oknozor](https://github.com/oknozor)

[107ac9](https://github.com/oknozor/toml-bombadil/commit/107ac944ec4252a2c161876304860ef2920b79f0) - 1.11.2 - [oknozor](https://github.com/oknozor)

[abb91c](https://github.com/oknozor/toml-bombadil/commit/abb91c214da714bfe40a1f380f2e103f6ea06690) - add cocogitto config - [oknozor](https://github.com/oknozor)


### Tests

[29be69](https://github.com/oknozor/toml-bombadil/commit/29be6918539c17936944e8fcdf1699b113b50e92) - add dockerized bats test suite - [oknozor](https://github.com/oknozor)


### Features

[babc78](https://github.com/oknozor/toml-bombadil/commit/babc78b4e9102b4ca63e831958e141506d6aa3fc) - add preinstall configuration clean up - [oknozor](https://github.com/oknozor)

[b49bdb](https://github.com/oknozor/toml-bombadil/commit/b49bdb85c808709f0184122380998c25e1d7cda3) - add scoped variables - [oknozor](https://github.com/oknozor)

[03a725](https://github.com/oknozor/toml-bombadil/commit/03a7250aa0d4553d61ab150457eb272fc6bb2870) - implement ignore glob pattern - [oknozor](https://github.com/oknozor)

[9a89a9](https://github.com/oknozor/toml-bombadil/commit/9a89a92dabb799362d5a3da7d9c2786174200f9e) - Add command for shell completions generation - [DSpeckhals](https://github.com/DSpeckhals)

[fa56af](https://github.com/oknozor/toml-bombadil/commit/fa56afa9af88081e1bf0261c7a110213900aaf14) - add profile flag for get command - [oknozor](https://github.com/oknozor)

[d51567](https://github.com/oknozor/toml-bombadil/commit/d515673b43a6df68d8a051ffcf681271f422515e) - add armored gpg values - [oknozor](https://github.com/oknozor)

[0de10b](https://github.com/oknozor/toml-bombadil/commit/0de10b113e667fd07d95c996af7ac41364f85121) - add get command for metadata - [DSpeckhals](https://github.com/DSpeckhals)

[9abac9](https://github.com/oknozor/toml-bombadil/commit/9abac997769aa993df163a25ec4db6bb905ecc6f) - add unlink command - [DSpeckhals](https://github.com/DSpeckhals)

[4d3510](https://github.com/oknozor/toml-bombadil/commit/4d35106bd57d118f6eba171d75d0761d53075459) - gpg dummy implementation - [oknozor](https://github.com/oknozor)


### Bug Fixes

[2e5753](https://github.com/oknozor/toml-bombadil/commit/2e57530307896b03106d1ebd6545a7399f23c01a) - Secrets are now correctly decryted and injected on install - [oknozor](https://github.com/oknozor)

[aa64d5](https://github.com/oknozor/toml-bombadil/commit/aa64d572edc079bb1fca3f4ab9c0dcf5e59039c6) - fix toc ordering - [oknozor](https://github.com/oknozor)

[3c16c8](https://github.com/oknozor/toml-bombadil/commit/3c16c82f5790d259b538c2ee34a8fb4747aa333d) - unlink command now correctly remove dots based on previous config - [oknozor](https://github.com/oknozor)

[b7cdd3](https://github.com/oknozor/toml-bombadil/commit/b7cdd356a1989e6cd2d54d8e280b099487d98255) - fix empty var files in dot overrides - [oknozor](https://github.com/oknozor)

[6f5d9b](https://github.com/oknozor/toml-bombadil/commit/6f5d9ba404ff0d6ed5bd43980cfa14c514d6c223) - fix default var path typo - [oknozor](https://github.com/oknozor)

[532ae9](https://github.com/oknozor/toml-bombadil/commit/532ae93d62b55c662e428e7de202449d5c88d0f8) - ignore all varfile for a dot across profile - [oknozor](https://github.com/oknozor)

[2c7dcc](https://github.com/oknozor/toml-bombadil/commit/2c7dcce0dfda0eed0e76feaa3e40100a148d0e38) - resolve % refence in dot entries - [oknozor](https://github.com/oknozor)

[8fe546](https://github.com/oknozor/toml-bombadil/commit/8fe546282d22e18b6b6738adfbb2bf2322fad416) - add gpg keypair to CI - [oknozor](https://github.com/oknozor)


- - -
## 1.11.2 - 2020-10-04


### Documentation

[8d52de](https://github.com/oknozor/toml-bombadil/commit/8d52de5a5b38fb39aaa14bbad41fd46fcee7dec0) - add readme instruction for secret vars - [oknozor](https://github.com/oknozor)

[bb3336](https://github.com/oknozor/toml-bombadil/commit/bb3336767029b7ad164fce1aa66306e5004a77a4) - update config example - [oknozor](https://github.com/oknozor)

[7099e6](https://github.com/oknozor/toml-bombadil/commit/7099e65d46f36268f9f06720ffcdd09f874b6024) - add documentation for config imports - [oknozor](https://github.com/oknozor)

[802ead](https://github.com/oknozor/toml-bombadil/commit/802eadbeabde605d1c0c7db95287f3546d810be8) - fix toc - [oknozor](https://github.com/oknozor)

[e9a1b3](https://github.com/oknozor/toml-bombadil/commit/e9a1b306d3a42917808f3d928b73804d7eb7c272) - add contribution guidelines - [oknozor](https://github.com/oknozor)

[53b07f](https://github.com/oknozor/toml-bombadil/commit/53b07fd7d821cd4e0f0761a48f3f01f2d1f7d69d) - update readme according to the new config format - [oknozor](https://github.com/oknozor)

[2e452e](https://github.com/oknozor/toml-bombadil/commit/2e452ea67f17554508ae091ea8116c2b5f34cd1e) - add TOC to readme - [oknozor](https://github.com/oknozor)

[5d9cda](https://github.com/oknozor/toml-bombadil/commit/5d9cdae5fafde53d8f56fddaaf2efb033a2a38ad) - typo - [oknozor](https://github.com/oknozor)

[3d2cc7](https://github.com/oknozor/toml-bombadil/commit/3d2cc7cd3abd2577c949000c8a3564b7272bcf7e) - update readme - [oknozor](https://github.com/oknozor)

[8f05f6](https://github.com/oknozor/toml-bombadil/commit/8f05f6c3002978707c0b8c21d87a609a18ec3955) - display help by default if no subcommand is provided - [oknozor](https://github.com/oknozor)

[516ee0](https://github.com/oknozor/toml-bombadil/commit/516ee082a15dd714d82b7d3c4d131bd2cb785ce1) - add aur badge - [oknozor](https://github.com/oknozor)

[93bbdb](https://github.com/oknozor/toml-bombadil/commit/93bbdb5c02a7b47f6ebbe0441af1c879ff9203c8) - update readme - [oknozor](https://github.com/oknozor)

[a0e125](https://github.com/oknozor/toml-bombadil/commit/a0e1253f8422bcc32182027180ef2c22f2ff7964) - move config examples to a dedicated dir - [oknozor](https://github.com/oknozor)

[e083ce](https://github.com/oknozor/toml-bombadil/commit/e083cecd214cca48c85adf9a2ae1d5be8e27c71f) - add codecove badge - [oknozor](https://github.com/oknozor)

[87c1c4](https://github.com/oknozor/toml-bombadil/commit/87c1c4a922011dc8d1b57726e44f461f44aff9b5) - rewrite some fn docs - [oknozor](https://github.com/oknozor)

[3cca80](https://github.com/oknozor/toml-bombadil/commit/3cca801227852569789b926ca9feeb12423fb919) - update readme - [oknozor](https://github.com/oknozor)

[63437a](https://github.com/oknozor/toml-bombadil/commit/63437af96c3aedbe02a9d54a167be4fa59cc3b9a) - basic documentation for public functions - [oknozor](https://github.com/oknozor)


### Tests

[1e33cb](https://github.com/oknozor/toml-bombadil/commit/1e33cb30521e1ee226a0e22f2a868037a3a3efee) - add test for var extension - [oknozor](https://github.com/oknozor)

[5cf2b5](https://github.com/oknozor/toml-bombadil/commit/5cf2b54c2caf4b7afbfde4a8e4a67b548def18f2) - add test for file and dire removal - [oknozor](https://github.com/oknozor)

[86efba](https://github.com/oknozor/toml-bombadil/commit/86efba3eb53d63ff34bbdef957576d2dbe2bcc86) - add test for profile switching - [oknozor](https://github.com/oknozor)

[6b211b](https://github.com/oknozor/toml-bombadil/commit/6b211b81d34eedc5aef384a7f70988e2b3b55209) - add test for path creation and conversion - [oknozor](https://github.com/oknozor)

[2a9c13](https://github.com/oknozor/toml-bombadil/commit/2a9c1338f1694b2f3897ea7c5b358c68a37684c0) - hook - [oknozor](https://github.com/oknozor)

[7f6d14](https://github.com/oknozor/toml-bombadil/commit/7f6d143469f8b30633acfaa90c3a7a8d1ed27f08) - fix failing test using absolute target path - [oknozor](https://github.com/oknozor)


### Refactoring

[0c9665](https://github.com/oknozor/toml-bombadil/commit/0c9665cfcb9b1c0020cc5b53515a9c09df9c888f) - clippy lints - [oknozor](https://github.com/oknozor)

[88e449](https://github.com/oknozor/toml-bombadil/commit/88e449b9c4f1882b19b5f7ff7422f983a175ccf9) - use fatal! macro on cli error - [oknozor](https://github.com/oknozor)

[17ddf4](https://github.com/oknozor/toml-bombadil/commit/17ddf4b504075df8422b626bcead049cf9cca0ab) - lints - [oknozor](https://github.com/oknozor)

[3aca8c](https://github.com/oknozor/toml-bombadil/commit/3aca8c8656269f1777a1bde75d114aad66c598b3) - refactor theming to allow writing generic preprocessor - [oknozor](https://github.com/oknozor)


### Features

[dd0ca4](https://github.com/oknozor/toml-bombadil/commit/dd0ca47eb86f8d41dd51d9569785facb62585782) - gpg dummy implementation - [oknozor](https://github.com/oknozor)

[1ac891](https://github.com/oknozor/toml-bombadil/commit/1ac8911e6ed2fb6fa9b355d7379c1a7ef81b30e3) - replace per dot profile with global profiles - [oknozor](https://github.com/oknozor)

[0927b5](https://github.com/oknozor/toml-bombadil/commit/0927b51c32d41fbdca4651d244d17956a535aae2) - replace meta vars with $ variable reference - [oknozor](https://github.com/oknozor)

[03e37e](https://github.com/oknozor/toml-bombadil/commit/03e37e38714c59a61d4b48731dbdf1468d3712e3) - change settings structure and add import - [oknozor](https://github.com/oknozor)

[955976](https://github.com/oknozor/toml-bombadil/commit/9559769cd4210ca4372b1f94791d00db3ef35247) - add profile switching - [oknozor](https://github.com/oknozor)

[07a61b](https://github.com/oknozor/toml-bombadil/commit/07a61b51bfce8fc372b40f122ae47cd5227387c9) - add meta vars - [oknozor](https://github.com/oknozor)

[137461](https://github.com/oknozor/toml-bombadil/commit/13746134abbabdf6d310c01d558ab5888602d3c5) - add color to 'link' command ouput - [oknozor](https://github.com/oknozor)

[46eaff](https://github.com/oknozor/toml-bombadil/commit/46eaff28b1e39795c5500257edf5df7c326ac562) - post install hooks - [oknozor](https://github.com/oknozor)

[306782](https://github.com/oknozor/toml-bombadil/commit/3067821ee429938f060a9201833124a1c77de06d) - complete rework - [oknozor](https://github.com/oknozor)

[5c9ace](https://github.com/oknozor/toml-bombadil/commit/5c9ace38540055ff69627cb72e23ad4eedea3e4f) - theme display - [oknozor](https://github.com/oknozor)


### Bug Fixes

[766fe3](https://github.com/oknozor/toml-bombadil/commit/766fe383659daf75fe9f0674b0491067fc33a5b9) - source file permissions are now applied to dot entries - [oknozor](https://github.com/oknozor)

[04ae9d](https://github.com/oknozor/toml-bombadil/commit/04ae9d66fcb89adf1421ac644e9fdd033684907f) - fix var path when activating profile - [oknozor](https://github.com/oknozor)

[d5fc0e](https://github.com/oknozor/toml-bombadil/commit/d5fc0e501f2509a2dd8e6d222601a85e4cb2d078) - add double quoted arg parsing - [oknozor](https://github.com/oknozor)

[9a5210](https://github.com/oknozor/toml-bombadil/commit/9a52105ddafeed4a948850dc53ff3222ef4001f8) - use % instead of $ for reference format - [oknozor](https://github.com/oknozor)

[b34b68](https://github.com/oknozor/toml-bombadil/commit/b34b68f06179f05447f92fd2c36cc3a39e14bcf8) - process remaining dots when one dot is not found - [oknozor](https://github.com/oknozor)

[e36872](https://github.com/oknozor/toml-bombadil/commit/e3687220b17ce3ad4456dcc3162b28d3e7f4bf20) - do not panic when var file is not found - [oknozor](https://github.com/oknozor)

[a6e019](https://github.com/oknozor/toml-bombadil/commit/a6e0196fc8574030b68a596970c289dee7bb6762) - fall back coping file in place when failing to read or parse file content - [oknozor](https://github.com/oknozor)

[1d3bf5](https://github.com/oknozor/toml-bombadil/commit/1d3bf55bd9f90f412a01706c97295dbfa2fd783a) - empty test directory - [oknozor](https://github.com/oknozor)

[733f07](https://github.com/oknozor/toml-bombadil/commit/733f07b2cf10eae50ebbe88ffe52884940dd826b) - fix pathes and theme creation, add lazy static config - [oknozor](https://github.com/oknozor)

[a8aa88](https://github.com/oknozor/toml-bombadil/commit/a8aa88a4824704c9338200be05483be5769e514a) - error handling - [oknozor](https://github.com/oknozor)


### Miscellaneous Chores

[2254dc](https://github.com/oknozor/toml-bombadil/commit/2254dcaeed27dedea1295317b0efb2348c2a7e2d) - add cocogitto config - [oknozor](https://github.com/oknozor)

[9df20c](https://github.com/oknozor/toml-bombadil/commit/9df20c7925061a1ce1892dc52385f66dbefa40c1) - move master to main - [oknozor](https://github.com/oknozor)

[9e621c](https://github.com/oknozor/toml-bombadil/commit/9e621c9d139fdb0d0a012c8f9b9c51f16299ef26) - add awesome logo - [oknozor](https://github.com/oknozor)

[e52281](https://github.com/oknozor/toml-bombadil/commit/e52281824b06836dabf1590140cf58b40e6be4e6) - fix clippy lint - [oknozor](https://github.com/oknozor)

[ec3215](https://github.com/oknozor/toml-bombadil/commit/ec3215385ca95f830597a09ec22c38f0dd07d3f5) - fmt all - [oknozor](https://github.com/oknozor)

[d7a030](https://github.com/oknozor/toml-bombadil/commit/d7a0302597a3c8c59c3d45b512d37da8efd88da8) - fmt all - [oknozor](https://github.com/oknozor)

[2ff52b](https://github.com/oknozor/toml-bombadil/commit/2ff52be9bac49a7e5ffa068a6c4e638f5395705f) - update issue templates - [oknozor](https://github.com/oknozor)

[3ff9bd](https://github.com/oknozor/toml-bombadil/commit/3ff9bd4ac46d82b5d33aabaf5a0b89745ce2d289) - version 1.2.0 - [oknozor](https://github.com/oknozor)

[dad19c](https://github.com/oknozor/toml-bombadil/commit/dad19c71597ed241c2f1deee16bd289847bed00d) - bump to 1.1.0 - [oknozor](https://github.com/oknozor)

[76b10e](https://github.com/oknozor/toml-bombadil/commit/76b10e59f0e5beafb3a1a98721221825d0780984) - version 1.0.0 - [oknozor](https://github.com/oknozor)

[1aa72d](https://github.com/oknozor/toml-bombadil/commit/1aa72d8c813689a0d8c3e33f5f542422460a0e2a) - cargo fmt * - [oknozor](https://github.com/oknozor)

[ab1f7d](https://github.com/oknozor/toml-bombadil/commit/ab1f7d544bd50138bb4e909a4680b5c6af62b6f8) - add MIT licence - [oknozor](https://github.com/oknozor)

[1615fb](https://github.com/oknozor/toml-bombadil/commit/1615fb37a2331811983e4e7f2d259c6ef2ab4db8) - prepare release with github action - [oknozor](https://github.com/oknozor)

[393212](https://github.com/oknozor/toml-bombadil/commit/39321263a272e07de7ddd2b5ced70c79f8d7681c) - add code coverage - [oknozor](https://github.com/oknozor)

[2dbb7d](https://github.com/oknozor/toml-bombadil/commit/2dbb7de0b38b9c7ac32a0ac1a95d32844c3594cc) - fmt * - [oknozor](https://github.com/oknozor)

[c52fc6](https://github.com/oknozor/toml-bombadil/commit/c52fc64358dfeceb509d5a6c82892221402fad0c) - add github action CI - [oknozor](https://github.com/oknozor)


- - -

This changelog was generated by [cocogitto](https://github.com/oknozor/cocogitto).