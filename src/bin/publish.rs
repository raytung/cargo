use cargo::core::Workspace;
use cargo::ops;
use cargo::util::{CliResult, Config};
use cargo::util::important_paths::find_root_manifest_for_wd;

#[derive(RustcDecodable)]
pub struct Options {
    flag_host: Option<String>,
    flag_token: Option<String>,
    flag_manifest_path: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_no_verify: bool,
    flag_allow_dirty: bool,
    flag_jobs: Option<u32>,
    flag_dry_run: bool,
}

pub const USAGE: &'static str = "
Upload a package to the registry

Usage:
    cargo publish [options]

Options:
    -h, --help               Print this message
    --host HOST              Host to upload the package to
    --token TOKEN            Token to use when uploading
    --no-verify              Don't verify package tarball before publish
    --allow-dirty            Allow publishing with a dirty source directory
    --manifest-path PATH     Path to the manifest of the package to publish
    -j N, --jobs N           Number of parallel jobs, defaults to # of CPUs
    --dry-run                Perform all checks without uploading
    -v, --verbose ...        Use verbose output
    -q, --quiet              No output printed to stdout
    --color WHEN             Coloring: auto, always, never

";

pub fn execute(options: Options, config: &Config) -> CliResult<Option<()>> {
    try!(config.configure_shell(options.flag_verbose,
                                options.flag_quiet,
                                &options.flag_color));
    let Options {
        flag_token: token,
        flag_host: host,
        flag_manifest_path,
        flag_no_verify: no_verify,
        flag_allow_dirty: allow_dirty,
        flag_jobs: jobs,
        flag_dry_run: dry_run,
        ..
    } = options;

    let root = try!(find_root_manifest_for_wd(flag_manifest_path.clone(), config.cwd()));
    let ws = try!(Workspace::new(&root, config));
    try!(ops::publish(&ws, &ops::PublishOpts {
        config: config,
        token: token,
        index: host,
        verify: !no_verify,
        allow_dirty: allow_dirty,
        jobs: jobs,
        dry_run: dry_run,
    }));
    Ok(None)
}
