use bindgen::RustTarget;
use bindgen::{callbacks, Bindings};
use camino::Utf8Path as Path;
use camino::Utf8PathBuf as PathBuf;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

/// All the libs that FFmpeg has
static LIBS: &'static [&'static str] = &[
        "libavcodec",
        "libavdevice",
        "libavfilter",
        "libavformat",
        "libavutil",
        "libswresample",
        "libswscale",
];

/// Whitelist of the headers we want to generate bindings
static HEADERS: Lazy<Vec<PathBuf>> = Lazy::new(|| {
    [
        "libavcodec/ac3_parser.h",
        "libavcodec/adts_parser.h",
        "libavcodec/avcodec.h",
        "libavcodec/avdct.h",
        "libavcodec/avfft.h",
        "libavcodec/bsf.h",
        "libavcodec/codec.h",
        "libavcodec/codec_desc.h",
        "libavcodec/codec_id.h",
        "libavcodec/codec_par.h",
        // "libavcodec/d3d11va.h",
        "libavcodec/defs.h",
        "libavcodec/dirac.h",
        "libavcodec/dv_profile.h",
        // "libavcodec/dxva2.h",
        "libavcodec/jni.h",
        "libavcodec/mediacodec.h",
        "libavcodec/packet.h",
        // "libavcodec/qsv.h",
        // "libavcodec/vdpau.h",
        "libavcodec/version.h",
        "libavcodec/version_major.h",
        // "libavcodec/videotoolbox.h",
        "libavcodec/vorbis_parser.h",
        // "libavcodec/xvmc.h",
        "libavdevice/avdevice.h",
        "libavdevice/version.h",
        "libavdevice/version_major.h",
        "libavfilter/avfilter.h",
        "libavfilter/buffersink.h",
        "libavfilter/buffersrc.h",
        "libavfilter/version.h",
        "libavfilter/version_major.h",
        "libavformat/avformat.h",
        "libavformat/avio.h",
        "libavformat/version.h",
        "libavformat/version_major.h",
        "libavutil/adler32.h",
        "libavutil/aes.h",
        "libavutil/aes_ctr.h",
        "libavutil/ambient_viewing_environment.h",
        "libavutil/attributes.h",
        "libavutil/audio_fifo.h",
        "libavutil/avassert.h",
        "libavutil/avconfig.h",
        "libavutil/avstring.h",
        "libavutil/avutil.h",
        "libavutil/base64.h",
        "libavutil/blowfish.h",
        "libavutil/bprint.h",
        "libavutil/bswap.h",
        "libavutil/buffer.h",
        "libavutil/camellia.h",
        "libavutil/cast5.h",
        "libavutil/channel_layout.h",
        "libavutil/common.h",
        "libavutil/cpu.h",
        "libavutil/crc.h",
        "libavutil/csp.h",
        "libavutil/des.h",
        "libavutil/detection_bbox.h",
        "libavutil/dict.h",
        "libavutil/display.h",
        "libavutil/dovi_meta.h",
        "libavutil/downmix_info.h",
        "libavutil/encryption_info.h",
        "libavutil/error.h",
        "libavutil/eval.h",
        "libavutil/executor.h",
        "libavutil/ffversion.h",
        "libavutil/fifo.h",
        "libavutil/file.h",
        "libavutil/film_grain_params.h",
        "libavutil/frame.h",
        "libavutil/hash.h",
        "libavutil/hdr_dynamic_metadata.h",
        "libavutil/hdr_dynamic_vivid_metadata.h",
        "libavutil/hmac.h",
        "libavutil/hwcontext.h",
        // "libavutil/hwcontext_cuda.h",
        // "libavutil/hwcontext_d3d11va.h",
        // "libavutil/hwcontext_drm.h",
        // "libavutil/hwcontext_dxva2.h",
        // "libavutil/hwcontext_mediacodec.h",
        // "libavutil/hwcontext_opencl.h",
        // "libavutil/hwcontext_qsv.h",
        // "libavutil/hwcontext_vaapi.h",
        // "libavutil/hwcontext_vdpau.h",
        // "libavutil/hwcontext_videotoolbox.h",
        // "libavutil/hwcontext_vulkan.h",
        "libavutil/imgutils.h",
        "libavutil/intfloat.h",
        "libavutil/intreadwrite.h",
        "libavutil/lfg.h",
        "libavutil/log.h",
        "libavutil/lzo.h",
        "libavutil/macros.h",
        "libavutil/mastering_display_metadata.h",
        "libavutil/mathematics.h",
        "libavutil/md5.h",
        "libavutil/mem.h",
        "libavutil/motion_vector.h",
        "libavutil/murmur3.h",
        "libavutil/opt.h",
        "libavutil/parseutils.h",
        "libavutil/pixdesc.h",
        "libavutil/pixelutils.h",
        "libavutil/pixfmt.h",
        "libavutil/random_seed.h",
        "libavutil/rational.h",
        "libavutil/rc4.h",
        "libavutil/replaygain.h",
        "libavutil/ripemd.h",
        "libavutil/samplefmt.h",
        "libavutil/sha.h",
        "libavutil/sha512.h",
        "libavutil/spherical.h",
        "libavutil/stereo3d.h",
        "libavutil/tea.h",
        "libavutil/threadmessage.h",
        "libavutil/time.h",
        "libavutil/timecode.h",
        "libavutil/timestamp.h",
        "libavutil/tree.h",
        "libavutil/twofish.h",
        "libavutil/tx.h",
        "libavutil/uuid.h",
        "libavutil/version.h",
        "libavutil/video_enc_params.h",
        "libavutil/video_hint.h",
        "libavutil/xtea.h",
        "libswresample/swresample.h",
        "libswresample/version.h",
        "libswresample/version_major.h",
        "libswscale/swscale.h",
        "libswscale/version.h",
        "libswscale/version_major.h",
    ]
    .into_iter()
    .map(|x| Path::new(x).into_iter().collect())
    .collect()
});

/// Filter out all symbols in the HashSet, and for others things it will act
/// exactly the same as `CargoCallback`.
#[derive(Debug)]
struct FilterCargoCallbacks {
    emitted_macro: HashSet<&'static str>,
}

impl FilterCargoCallbacks {
    fn new(set: HashSet<&'static str>) -> Self {
        Self { emitted_macro: set }
    }
}

impl callbacks::ParseCallbacks for FilterCargoCallbacks {
    fn will_parse_macro(&self, name: &str) -> callbacks::MacroParsingBehavior {
        if self.emitted_macro.contains(name) {
            callbacks::MacroParsingBehavior::Ignore
        } else {
            callbacks::MacroParsingBehavior::Default
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FFmpegLinkMode {
    Static,
    Dynamic,
}

#[cfg(not(target_os = "windows"))]
impl FFmpegLinkMode {
    fn is_static(&self) -> bool {
        self == &Self::Static
    }
}

impl From<String> for FFmpegLinkMode {
    fn from(value: String) -> Self {
        match &*value {
            "static" => FFmpegLinkMode::Static,
            "dynamic" => FFmpegLinkMode::Dynamic,
            _ => panic!("Invalid FFMPEG_LINK_MODE value, expected [static,dynamic]"),
        }
    }
}

impl std::fmt::Display for FFmpegLinkMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FFmpegLinkMode::Static => write!(f, "static"),
            FFmpegLinkMode::Dynamic => write!(f, "dylib"),
        }
    }
}

fn generate_bindings(ffmpeg_include_dir: &Path, headers: &[PathBuf]) -> Bindings {
    if !Path::new(ffmpeg_include_dir).exists() {
        panic!(
            "FFmpeg include dir: `{:?}` doesn't exits",
            ffmpeg_include_dir
        );
    }
    // Because of the strange `FP_*` in `math.h` https://github.com/rust-lang/rust-bindgen/issues/687
    let filter_callback = FilterCargoCallbacks::new(
        [
            "FP_NAN",
            "FP_INFINITE",
            "FP_ZERO",
            "FP_SUBNORMAL",
            "FP_NORMAL",
        ]
        .into_iter()
        .collect(),
    );

    // Bindgen on all avaiable headers
    headers
        .iter()
        .map(|header| ffmpeg_include_dir.join(header))
        .filter(|path| {
            let exists = Path::new(&path).exists();
            if !exists {
                eprintln!("Header path `{:?}` not found.", path);
            }
            exists
        })
        .fold(
            {
                bindgen::builder()
                    // Force impl Debug if possible(for `AVCodecParameters`)
                    .impl_debug(true)
                    .rust_target(RustTarget::stable(68, 0).ok().unwrap())
                    .parse_callbacks(Box::new(filter_callback))
                    // Add clang path, for `#include` header finding in bindgen process.
                    .clang_arg(format!("-I{}", ffmpeg_include_dir))
                    // Workaround: https://github.com/rust-lang/rust-bindgen/issues/2159
                    .blocklist_type("__mingw_ldbl_type_t")
                    // Stop bindgen from prefixing enums
                    .prepend_enum_name(false)
            },
            |builder, header| builder.header(header),
        )
        .generate()
        .expect("Binding generation failed.")
}

#[allow(dead_code)]
pub struct EnvVars {
    target: String,
    docs_rs: Option<String>,
    out_dir: PathBuf,
    num_jobs: String,
    ffmpeg_configuration: Vec<String>,
    ffmpeg_link_mode: FFmpegLinkMode,
    ffmpeg_rockchip_mpp: bool,
}

impl EnvVars {
    fn init() -> Self {
        println!("cargo:rerun-if-env-changed=DOCS_RS");
        println!("cargo:rerun-if-env-changed=OUT_DIR");
        println!("cargo:rerun-if-env-changed=FFMPEG_CONFIGURATION");
        println!("cargo:rerun-if-env-changed=FFMPEG_LINK_MODE");
        println!("cargo:rerun-if-env-changed=FFMPEG_ROCKCHIP_MPP");
        Self {
            target: env::var("TARGET").expect("TARGET env var"),
            docs_rs: env::var("DOCS_RS").ok(),
            out_dir: remove_verbatim(env::var("OUT_DIR").expect("OUT_DIR env var")),
            num_jobs: env::var("NUM_JOBS").expect("NUM_JOBS env var"),
            ffmpeg_configuration: env::var("FFMPEG_CONFIGURATION").expect("FFMPEG_CONFIGURATION env var")
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(str::trim)
                .map(String::from)
                .collect(),
            ffmpeg_link_mode: env::var("FFMPEG_LINK_MODE").ok()
                .map(Into::into)
                .unwrap_or(FFmpegLinkMode::Static),
            ffmpeg_rockchip_mpp: env::var("FFMPEG_ROCKCHIP_MPP")
                .map(|v| v.trim().parse().unwrap_or(false)).unwrap_or(false),
        }
    }
}

/// clang doesn't support -I{verbatim path} on windows, so we need to remove it if possible.
fn remove_verbatim(path: String) -> PathBuf {
    let path = if let Some(path) = path.strip_prefix(r#"\\?\"#) {
        path.to_string()
    } else {
        path
    };
    PathBuf::from(path)
}

#[cfg(not(target_os = "windows"))]
mod pkg_config_linking {
    use super::*;

    /// Returns error when some library are missing. Otherwise, returns the paths of the libraries.
    ///
    /// Note: no side effect if this function errors.
    pub fn linking_with_pkg_config(
        library_names: &[&str],
        statik: bool,
    ) -> Result<Vec<PathBuf>, pkg_config::Error> {
        // dry run for library linking
        for libname in library_names {
            pkg_config::Config::new()
                .statik(statik)
                .cargo_metadata(false)
                .env_metadata(false)
                .print_system_libs(false)
                .print_system_cflags(false)
                .probe(libname)?;
        }

        // real linking
        let mut paths = HashSet::new();
        for libname in library_names {
            let new_paths = pkg_config::Config::new()
                .statik(statik)
                .probe(libname)
                .unwrap_or_else(|_| panic!("{} not found!", libname))
                .include_paths;
            for new_path in new_paths {
                let new_path = new_path.to_str().unwrap().to_string();
                paths.insert(new_path);
            }
        }
        Ok(paths.into_iter().map(PathBuf::from).collect())
    }
}

fn linking(
    env_vars: &EnvVars,
    ffmpeg_include_dir: &Path,
    pkg_config_path: &str,
) {
    let output_binding_path = &env_vars.out_dir.join("binding.rs");

    #[cfg(not(target_os = "windows"))]
    {
        fn linking_with_pkg_config_and_bindgen(
            env_vars: &EnvVars,
            ffmpeg_include_dir: &Path,
            output_binding_path: &Path,
        ) -> Result<(), pkg_config::Error> {
            // Probe libraries(enable emitting cargo metadata)
            pkg_config_linking::linking_with_pkg_config(
                LIBS,
                env_vars.ffmpeg_link_mode.is_static(),
            )?;
            generate_bindings(ffmpeg_include_dir, &HEADERS)
                .write_to_file(output_binding_path)
                .expect("Cannot write binding to file.");
            Ok(())
        }
        // Detect if we are inside a nix shell
        if env::var("PKG_CONFIG_PATH_FOR_TARGET").is_ok() {
            env::set_var("PKG_CONFIG_PATH_FOR_TARGET", pkg_config_path);
        } else {
            env::set_var("PKG_CONFIG_PATH", pkg_config_path);
        }
        if env::var("CROSS_SYSROOT").is_ok() {
            // cross > 0.2.5 do not work without this variable
            // env::set_var("PKG_CONFIG_SYSROOT_DIR", cross_sysroot);
            env::set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        }
        linking_with_pkg_config_and_bindgen(&env_vars, ffmpeg_include_dir, output_binding_path)
            .expect("Static linking with pkg-config failed.");
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(ffmpeg_libs_dir) = env_vars.ffmpeg_libs_dir.as_ref() {
            linking_with_libs_dir(
                LIBS,
                ffmpeg_libs_dir,
                env_vars.ffmpeg_link_mode.unwrap_or(FFmpegLinkMode::Static),
            );
            if let Some(ffmpeg_binding_path) = env_vars.ffmpeg_binding_path.as_ref() {
                use_prebuilt_binding(ffmpeg_binding_path, output_binding_path);
            } else if let Some(ffmpeg_include_dir) = env_vars.ffmpeg_include_dir.as_ref() {
                generate_bindings(ffmpeg_include_dir, &HEADERS)
                    .write_to_file(output_binding_path)
                    .expect("Cannot write binding to file.");
            } else {
                panic!("No binding generation method is set!");
            }
        } else {
            #[cfg(feature = "link_vcpkg_ffmpeg")]
            vcpkg_linking::linking_with_vcpkg_and_bindgen(&env_vars, output_binding_path)
                .expect("Linking FFmpeg with vcpkg failed.");
            #[cfg(not(feature = "link_vcpkg_ffmpeg"))]
            panic!(
                "
!!!!!!! rusty_ffmpeg: No linking method set!
Use FFMPEG_LIBS_DIR if you have prebuilt FFmpeg libraries.
Enable `link_vcpkg_ffmpeg` feature if you want to link ffmpeg provided by vcpkg.
"
            );
        }
    }
}

fn build_all(env_vars: &EnvVars) -> (PathBuf, String) {
    println!("Building FFmpeg. Current dir: {:?}", env::current_dir());

    let mut pkg_config_dirs = vec!();
    let mut include_dirs = vec!();
    let mut shared_lib_cleanup_dirs = vec!();

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH env var");
    println!("Target arch: {target_arch}");

    let cross_toolchain_prefix = env::var("CROSS_TOOLCHAIN_PREFIX").unwrap_or("".to_string());
    let (meson_cross_path, ffmpeg_cross_opts) = if !cross_toolchain_prefix.is_empty() {
        let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS env var");
        let target_os = match target_os.as_str() {
            "windows" => "mingw32",
            os => os,
        };
        println!("Target os: {target_os}");
        let cpu_arch = match target_arch.as_str() {
            "aarch64" => "armv8-a",
            "arm" => "armv7-a",
            _ => &target_arch
        };
        println!("CPU arch: {cpu_arch:?}");

        let meson_cross_path = env_vars.out_dir.join("meson_cross.txt");
        let mut meson_cross_file = File::create(&meson_cross_path)
            .expect("Failed to create meson_cross.txt file");
        meson_cross_file.write_all(
            indoc::formatdoc! {"
                [binaries]
                c = '{cross_toolchain_prefix}gcc'
                cpp = '{cross_toolchain_prefix}g++'
                ar = '{cross_toolchain_prefix}ar'
                strip = '{cross_toolchain_prefix}strip'

                [host_machine]
                system = 'linux'
                cpu_family = 'x86_64'
                cpu = 'x86_64'
                endian = 'little'

                [properties]
                needs_exe_wrapper = true
            "}.as_bytes()
        ).expect("Failed to write meson_cross.txt file");
        let mut ffmpeg_cross_opts = vec!();
        ffmpeg_cross_opts.extend_from_slice(&[
            "--enable-cross-compile".to_string(),
            format!("--cc={cross_toolchain_prefix}gcc"),
            format!("--cxx={cross_toolchain_prefix}g++"),
            format!("--ld={cross_toolchain_prefix}g++"),
            format!("--ar={cross_toolchain_prefix}ar"),
            format!("--strip={cross_toolchain_prefix}strip"),
            format!("--arch={target_arch}"),
            format!("--target-os={target_os}"),
        ]);
        if cpu_arch != target_arch {
            ffmpeg_cross_opts.push(format!("--cpu={cpu_arch}"));
        }
        (
            Some(meson_cross_path),
            Some(ffmpeg_cross_opts),
        )
    } else {
        (None, None)
    };

    let cmake_toolchain_path = env::var(
        format!("CMAKE_TOOLCHAIN_FILE_{}", env_vars.target.replace("-", "_"))
    ).ok();

    if env_vars.ffmpeg_rockchip_mpp {
        build_libdrm(
            env_vars,
            meson_cross_path.as_deref(),
            &mut pkg_config_dirs,
            &mut shared_lib_cleanup_dirs,
        );

        build_rockchip_librga(
            env_vars,
            meson_cross_path.as_deref(),
            &mut pkg_config_dirs,
        );

        build_rockchip_mpp(
            env_vars,
            cmake_toolchain_path.as_deref(),
            &mut pkg_config_dirs,
            &mut shared_lib_cleanup_dirs,
        );
    }

    if env_vars.ffmpeg_configuration.iter().any(|v| v == "--enable-libvpl") {
        build_libvpl(env_vars, cmake_toolchain_path.as_deref(), &mut pkg_config_dirs);
    }

    if env_vars.ffmpeg_configuration.iter().any(|v| v == "--enable-amf") {
        build_amf(env_vars, &mut include_dirs);
    }

    if env_vars.ffmpeg_configuration.iter().any(|v| v == "--enable-ffnvcodec") {
        build_ffnvcodec(env_vars, &mut pkg_config_dirs);
    }

    let ffmpeg_include_dir = build_ffmpeg(
        env_vars,
        ffmpeg_cross_opts.as_deref(),
        &mut include_dirs,
        &mut pkg_config_dirs,
    );

    for cleanup_shared_libs_dir in &shared_lib_cleanup_dirs {
        // FIXME: Find out a way how to force a static linking
        for shared_lib_file_entry in fs::read_dir(cleanup_shared_libs_dir)
            .expect("Cannot read directory with shared libs for removing")
        {
            let shared_lib_file_path = shared_lib_file_entry
                .expect("Cannot get shared lib entry")
                .path();
            let shared_lib_file_name = shared_lib_file_path.file_name()
                .expect("Missing shared lib file name")
                .to_string_lossy();
            if shared_lib_file_name.ends_with(".so") || shared_lib_file_name.contains(".so.") {
                fs::remove_file(&shared_lib_file_path)
                    .expect(&format!("Failed to remove {shared_lib_file_path:?} file"));
            }
        }
    }

    (
        ffmpeg_include_dir,
        mk_pkg_config_path(&pkg_config_dirs),
    )
}

fn mk_pkg_config_path(pkg_config_dirs: &[PathBuf]) -> String {
    pkg_config_dirs.iter()
        .map(|p| p.as_str())
        .collect::<Vec<_>>()
        .join(":")
}

fn build_libdrm(
    env_vars: &EnvVars,
    meson_cross_path: Option<&Path>,
    pkg_config_dirs: &mut Vec<PathBuf>,
    shared_lib_cleanup_dirs: &mut Vec<PathBuf>,
) {
    let libdrm_out_dir = env_vars.out_dir.join("libdrm");
    let libdrm_build_dir = libdrm_out_dir.join("meson");
    let libdrm_install_dir = libdrm_out_dir.join("install");
    let libdrm_pkg_config_path = libdrm_install_dir.join("lib").join("pkgconfig");
    let mut libdrm_setup_cmd = Command::new("meson");
    libdrm_setup_cmd
        .args([
            "setup", "vendor/libdrm", libdrm_build_dir.as_str(),
        ]);
    if let Some(meson_cross_path) = &meson_cross_path {
        libdrm_setup_cmd
            .args(["--cross-file", meson_cross_path.as_str()]);
    }
    libdrm_setup_cmd
        .args([
            // "--wipe",
            "--prefix", libdrm_install_dir.as_str(),
            "--libdir=lib",
            "--buildtype=release",
            "--default-library=static",
            "-Dintel=disabled",
            "-Dradeon=disabled",
            "-Damdgpu=disabled",
            "-Dnouveau=disabled",
            "-Dvmwgfx=disabled",
        ]);
    let libdrm_setup_status = libdrm_setup_cmd
        .status()
        .expect("Failed to run libdrm setup");
    assert!(libdrm_setup_status.success(), "Error setting up libdrm");
    let libdrm_configure_status = Command::new("meson")
        .args(["configure", libdrm_build_dir.as_str()])
        .status()
        .expect("Failed to run libdrm configuration");
    assert!(libdrm_configure_status.success(), "Error configuring libdrm");
    let libdrm_build_status = Command::new("ninja")
        .args(["-C", libdrm_build_dir.as_str(), "install"])
        .status()
        .expect("Failed to run libdrm building");
    assert!(libdrm_build_status.success(), "Error building libdrm");

    pkg_config_dirs.push(libdrm_pkg_config_path);
    shared_lib_cleanup_dirs.push(libdrm_install_dir.join("lib"));
}

fn build_rockchip_librga(
    env_vars: &EnvVars,
    meson_cross_path: Option<&Path>,
    pkg_config_dirs: &mut Vec<PathBuf>,
) {
    let rockchip_librga_out_dir = env_vars.out_dir.join("rockchip-librga");
    let rockchip_librga_build_dir = rockchip_librga_out_dir.join("meson");
    let rockchip_librga_install_dir = rockchip_librga_out_dir.join("install");
    let rockchip_librga_pkg_config_path = rockchip_librga_install_dir.join("lib").join("pkgconfig");
    let mut rockchip_librga_setup_cmd = Command::new("meson");
    rockchip_librga_setup_cmd
        .args([
            "setup", "vendor/rockchip-librga", rockchip_librga_build_dir.as_str(),
        ]);
    if let Some(meson_cross_path) = &meson_cross_path {
        rockchip_librga_setup_cmd
            .args(["--cross-file", meson_cross_path.as_str()]);
    }
    rockchip_librga_setup_cmd
        .args([
            // "--wipe",
            "--prefix", rockchip_librga_install_dir.as_str(),
            "--libdir=lib",
            "--buildtype=release",
            "--default-library=static",
            "-Dcpp_args=-fpermissive",
            "-Dlibdrm=false",
            "-Dlibrga_demo=false",
            "-Dbuild_test=false",
        ]);
    let rockchip_librga_setup_status = rockchip_librga_setup_cmd
        .status()
        .expect("Failed to run rockchip-librga setup");
    assert!(rockchip_librga_setup_status.success(), "Error setting up rockchip-librga");
    let rockchip_librga_configure_status = Command::new("meson")
        .args(["configure", rockchip_librga_build_dir.as_str()])
        .status()
        .expect("Failed to run rockchip-librga configuration");
    assert!(rockchip_librga_configure_status.success(), "Error configuring rockchip-librga");
    let rockchip_librga_build_status = Command::new("ninja")
        .args(["-C", rockchip_librga_build_dir.as_str(), "install"])
        .status()
        .expect("Failed to run rockchip-librga building");
    assert!(rockchip_librga_build_status.success(), "Error building rockchip-librga");

    pkg_config_dirs.push(rockchip_librga_pkg_config_path);
}

fn build_rockchip_mpp(
    env_vars: &EnvVars,
    cmake_toolchain_path: Option<&str>,
    pkg_config_dirs: &mut Vec<PathBuf>,
    shared_lib_cleanup_dirs: &mut Vec<PathBuf>,
) {
    let rockchip_mpp_out_dir = env_vars.out_dir.join("rockchip-mpp");
    let rockchip_mpp_build_dir = rockchip_mpp_out_dir.join("cmake");
    let rockchip_mpp_install_dir = rockchip_mpp_out_dir.join("install");
    let rockchip_mpp_pkg_config_path = rockchip_mpp_install_dir.join("lib").join("pkgconfig");
    let mut rockchip_mpp_configure_cmd = Command::new("cmake");
    rockchip_mpp_configure_cmd
        .arg("-GNinja")
        .arg("-DBUILD_TEST=false")
        .arg(format!("-DCMAKE_INSTALL_PREFIX={rockchip_mpp_install_dir}"))
        .arg(format!("-Svendor/rockchip-mpp"))
        .arg(format!("-B{rockchip_mpp_build_dir}"));
    if let Some(cmake_toolchain_path) = cmake_toolchain_path {
        rockchip_mpp_configure_cmd
            .args(["--toolchain", cmake_toolchain_path]);
    }
    let rockchip_mpp_configure_status = rockchip_mpp_configure_cmd
        .status()
        .expect("Failed to run rockchip-mpp configuration");
    assert!(rockchip_mpp_configure_status.success(), "Error configuring rockchip-mpp");
    let rockchip_mpp_build_status = Command::new("ninja")
        .args([
            "-C", rockchip_mpp_build_dir.as_str(),
            "install",
        ])
        .status()
        .expect("Failed to run rockchip-mpp building");
    assert!(rockchip_mpp_build_status.success(), "Error building rockchip-mpp");

    pkg_config_dirs.push(rockchip_mpp_pkg_config_path);
    shared_lib_cleanup_dirs.push(rockchip_mpp_install_dir.join("lib"));
}

fn build_ffnvcodec(env_vars: &EnvVars, pkg_config_dirs: &mut Vec<PathBuf>) {
    let ffnvcodec_out_dir = env_vars.out_dir.join("ffnvcodec");
    let ffnvcodec_install_dir = ffnvcodec_out_dir.join("install");
    let ffnvcodec_build_status = Command::new("make")
        .args([
            "-C", "vendor/ffnvcodec"
        ])
        .status()
        .expect("Failed to build ffnvcodec");
    assert!(ffnvcodec_build_status.success(), "Error building rockchip-mpp");
    let ffnvcodec_install_status = Command::new("make")
        .args([
            "-C", "vendor/ffnvcodec",
            "install",
        ])
        .arg(format!("PREFIX={ffnvcodec_install_dir}"))
        .status()
        .expect("Failed to install ffnvcodec");
    assert!(ffnvcodec_install_status.success(), "Error building rockchip-mpp");

    pkg_config_dirs.push(ffnvcodec_install_dir.join("lib").join("pkgconfig"));
}

fn build_libvpl(
    env_vars: &EnvVars,
    cmake_toolchain_path: Option<&str>,
    pkg_config_dirs: &mut Vec<PathBuf>,
) {
    let out_dir = env_vars.out_dir.join("libvpl");
    let build_dir = out_dir.join("cmake");
    let install_dir = out_dir.join("install");
    let pkg_config_path = install_dir.join("lib").join("pkgconfig");
    let mut configure_cmd = Command::new("cmake");
    configure_cmd
        .arg("-GNinja")
        .arg("-DBUILD_TEST=false")
        .arg(format!("-DCMAKE_INSTALL_PREFIX={install_dir}"))
        .arg(format!("-DBUILD_SHARED_LIBS=OFF"))
        .arg(format!("-Svendor/libvpl"))
        .arg(format!("-B{build_dir}"));
    if let Some(cmake_toolchain_path) = cmake_toolchain_path {
        configure_cmd
            .args(["--toolchain", cmake_toolchain_path]);
    }
    let configure_status = configure_cmd
        .status()
        .expect("Failed to run rockchip-mpp configuration");
    assert!(configure_status.success(), "Error configuring libvpl");
    let build_status = Command::new("ninja")
        .args([
            "-C", build_dir.as_str(),
            "install",
        ])
        .status()
        .expect("Failed to run libvpl building");
    assert!(build_status.success(), "Error building libvpl");

    pkg_config_dirs.push(pkg_config_path);
}

fn build_amf(
    env_vars: &EnvVars,
    include_dirs: &mut Vec<PathBuf>,
) {
    let out_dir = env_vars.out_dir.join("amf");
    let install_dir = out_dir.join("install");
    let include_dir = install_dir.join("include");
    fs::create_dir_all(&include_dir)
        .expect("Cannot create include directory for amf headers");

    let copy_status = Command::new("cp")
        .arg("-r")
        .arg("vendor/amf/amf/public/include")
        .arg(include_dir.join("AMF"))
        .status()
        .expect("Failed to copy amg headers");
    assert!(copy_status.success(), "Error copying amf headers");

    include_dirs.push(include_dir);
}

fn build_ffmpeg(
    env_vars: &EnvVars,
    cross_opts: Option<&[String]>,
    include_dirs: &[PathBuf],
    pkg_config_dirs: &mut Vec<PathBuf>,
) -> PathBuf {
    let ffmpeg_out_dir = env_vars.out_dir.join("ffmpeg");
    println!("FFmpeg output directory: {ffmpeg_out_dir:?}");
    println!("FFmpeg pkg config path: {pkg_config_dirs:?}");
    let ffmpeg_src_dir = ffmpeg_out_dir.join("src");
    if !ffmpeg_src_dir.join("configure").exists() {
        // We clone ffmpeg sources as ffmpeg produces build artifacts
        // right in the source directory
        let mut ffmpeg_git_clone_cmd = Command::new("git");
        ffmpeg_git_clone_cmd.args(["clone", "vendor/ffmpeg", ffmpeg_src_dir.as_str()]);
        assert!(
            ffmpeg_git_clone_cmd.status()
                                .expect("Failed to run git clone for ffmpeg")
                                .success(),
            "Failed to clone ffmpeg sources"
        );
    }
    let ffmpeg_install_dir = ffmpeg_out_dir.join("install");
    let mut ffmpeg_configure_cmd = Command::new(
        ffmpeg_src_dir.join("configure")
    );
    ffmpeg_configure_cmd.current_dir(&ffmpeg_src_dir)
        .arg(format!("--prefix={ffmpeg_install_dir}"))
        .args([
            "--enable-gpl",
            "--enable-version3",
            "--disable-iconv",
            "--disable-zlib",
            "--disable-everything",
            "--disable-programs",
            "--disable-doc",
            "--fatal-warnings",
        ]);

    if let Some(ffmpeg_cross_opts) = cross_opts {
        ffmpeg_configure_cmd
            .args(ffmpeg_cross_opts);
    }

    if !include_dirs.is_empty() {
        for include_dir in include_dirs {
            ffmpeg_configure_cmd
                .arg(format!("--extra-cflags=-I{include_dir}"));
        }
    }

    // Detect if we are inside a nix shell
    let ffmpeg_pkg_config_path = mk_pkg_config_path(&pkg_config_dirs);
    if let Ok(pkg_config_path) = env::var("PKG_CONFIG_PATH_FOR_TARGET") {
        ffmpeg_configure_cmd.env(
            "PKG_CONFIG_PATH_FOR_TARGET",
            format!("{pkg_config_path}:{ffmpeg_pkg_config_path}")
        );
    } else {
        let pkg_config_path = env::var("PKG_CONFIG_PATH").unwrap_or("".to_string());
        ffmpeg_configure_cmd.env(
            "PKG_CONFIG_PATH",
            format!("{pkg_config_path}:{ffmpeg_pkg_config_path}")
        );
    };
    ffmpeg_configure_cmd.args(&env_vars.ffmpeg_configuration);
    println!("FFmpeg configure command: {ffmpeg_configure_cmd:?}");
    assert!(
        ffmpeg_configure_cmd.status()
            .expect("Failed to run ffmpeg configuration")
            .success(),
        "Error configuring ffmpeg"
    );
    let ffmpeg_build_status = Command::new("make")
        .args([
            "-C", ffmpeg_src_dir.as_str(),
            "-j", &env_vars.num_jobs,
        ])
        .status()
        .expect("Failed to build ffmpeg");
    assert!(ffmpeg_build_status.success(), "Error building ffmpeg");
    let ffmpeg_install_status = Command::new("make")
        .args(["-C", ffmpeg_src_dir.as_str()])
        .arg("install")
        .status()
        .expect("Failed to run ffmpeg installation");
    assert!(ffmpeg_install_status.success(), "Error installing ffmpeg");

    pkg_config_dirs.push(
        ffmpeg_install_dir.join("lib").join("pkgconfig")
    );

    ffmpeg_install_dir.join("include")
}

fn main() {
    let env_vars = EnvVars::init();

    let (include_dir, pkg_config_path) = build_all(&env_vars);
    println!("FFmpeg include dir: {}", include_dir);
    println!("FFmpeg pkg-config path: {}", pkg_config_path);

    linking(&env_vars, &include_dir, &pkg_config_path);

    // https://github.com/rust-lang/rust/issues/37403#issuecomment-2061712123
    println!("cargo:rustc-link-lib=static:-bundle=stdc++");
}
