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

fn use_prebuilt_binding(from: &Path, to: &Path) {
    fs::copy(from, to).expect("Prebuilt binding file failed to be copied.");
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

fn linking_with_libs_dir(library_names: &[&str], ffmpeg_libs_dir: &Path, mode: FFmpegLinkMode) {
    println!("cargo:rustc-link-search=native={ffmpeg_libs_dir}");
    for library_name in library_names {
        println!("cargo:rustc-link-lib={mode}={library_name}");
    }
}

fn linking_with_single_lib(library_name: &str, ffmpeg_lib_dir: &Path, mode: FFmpegLinkMode) {
    println!("cargo:rustc-link-search=native={ffmpeg_lib_dir}");
    println!("cargo:rustc-link-lib={mode}={library_name}");
}

#[allow(dead_code)]
pub struct EnvVars {
    target: String,
    docs_rs: Option<String>,
    out_dir: PathBuf,
    num_jobs: String,
    ffmpeg_configuration: Vec<String>,
    ffmpeg_link_mode: Option<FFmpegLinkMode>,
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
                .map(String::from)
                .collect(),
            ffmpeg_link_mode: env::var("FFMPEG_LINK_MODE").ok().map(Into::into),
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
    ffmpeg_pkg_config_path: &str,
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
            let include_paths = pkg_config_linking::linking_with_pkg_config(
                LIBS,
                env_vars
                    .ffmpeg_link_mode
                    .map(|x| x.is_static())
                    .unwrap_or_default(),
            )?;
            generate_bindings(ffmpeg_include_dir, &HEADERS)
                .write_to_file(output_binding_path)
                .expect("Cannot write binding to file.");
            Ok(())
        }
        // Hint: set PKG_CONFIG_PATH to some placeholder value will let pkg_config probing system library.
        // if !Path::new(ffmpeg_pkg_config_path).exists() {
        //     panic!(
        //         "error: FFMPEG_PKG_CONFIG_PATH is set to `{}`, which does not exist.",
        //         ffmpeg_pkg_config_path
        //     );
        // }
        // Detect if we are inside a nix shell
        if env::var("PKG_CONFIG_PATH_FOR_TARGET").is_ok() {
            env::set_var("PKG_CONFIG_PATH_FOR_TARGET", ffmpeg_pkg_config_path);
        } else {
            env::set_var("PKG_CONFIG_PATH", ffmpeg_pkg_config_path);
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

fn build_ffmpeg(env_vars: &EnvVars) -> (PathBuf, String) {
    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS env var");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH env var");
    let cpu_arch = match target_arch.as_str() {
        "aarch64" => "armv8-a",
        _ => &target_arch
    };

    let (meson_cross_path, ffmpeg_cross_opts) =
        if let Ok(cross_toolchain_prefix) = env::var("CROSS_TOOLCHAIN_PREFIX")
    {
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
        (
            Some(meson_cross_path),
            Some([
                "--enable-cross-compile".to_string(),
                format!("--cc={cross_toolchain_prefix}gcc"),
                format!("--cxx={cross_toolchain_prefix}g++"),
                format!("--ld={cross_toolchain_prefix}g++"),
                format!("--ar={cross_toolchain_prefix}ar"),
                format!("--strip={cross_toolchain_prefix}strip"),
                format!("--cpu={cpu_arch}"),
                format!("--target-os={target_os}"),
                format!("--arch={target_arch}"),
            ])
        )
    } else {
        (None, None)
    };

    let cmake_toolchain_path = env::var(
        format!("CMAKE_TOOLCHAIN_FILE_{}", env_vars.target.replace("-", "_"))
    ).ok();

    let (ffmpeg_pkg_config_path, cleanup_files) = if env_vars.ffmpeg_rockchip_mpp {
        let rockchip_librga_out_dir = env_vars.out_dir.join("rockchip-librga");
        let rockchip_librga_build_dir = rockchip_librga_out_dir.join("meson");
        let rockchip_librga_install_dir = rockchip_librga_out_dir.join("install");
        let rockchip_librga_pkg_config_path = rockchip_librga_install_dir.join("lib").join("pkgconfig");
        let mut rockchip_librga_setup_cmd = Command::new("meson");
        rockchip_librga_setup_cmd
            .args([
                "setup", "vendor/rockchip-librga", rockchip_librga_build_dir.as_str(),
            ]);
        if let Some(meson_cross_path) = meson_cross_path {
            rockchip_librga_setup_cmd
                .args(["--cross-file", meson_cross_path.as_str()]);
        }
        rockchip_librga_setup_cmd
            .args([
                "--prefix", rockchip_librga_install_dir.as_str(),
                "--libdir=lib",
                "--buildtype=release",
                "--default-library=static",
                "-Dcpp_args=-fpermissive",
                "-Dlibdrm=false",
                "-Dlibrga_demo=false",
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

        let rockchip_mpp_out_dir = env_vars.out_dir.join("rockchip-mpp");
        let rockchip_mpp_build_dir = rockchip_mpp_out_dir.join("cmake");
        let rockchip_mpp_install_dir = rockchip_mpp_out_dir.join("install");
        let rockchip_mpp_pkg_config_path = rockchip_mpp_install_dir.join("lib").join("pkgconfig");
        let mut rockchip_mpp_configure_cmd = Command::new("cmake");
        rockchip_mpp_configure_cmd
            .arg("-GNinja")
            .arg(format!("-DCMAKE_INSTALL_PREFIX={rockchip_mpp_install_dir}"))
            .arg(format!("-Svendor/rockchip-mpp"))
            .arg(format!("-B{rockchip_mpp_build_dir}"));
        if let Some(cmake_toolchain_path) = cmake_toolchain_path {
            rockchip_mpp_configure_cmd
                .args(["--toolchain", &cmake_toolchain_path]);
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

        // println!("cargo:rustc-link-lib={}", rockchip_mpp_install_dir.join("lib").join("librockchip_mpp.a"));

        (
            Some(format!(
                "{rockchip_mpp_pkg_config_path}:{rockchip_librga_pkg_config_path}"
            )),
            vec!(
                rockchip_mpp_install_dir.join("lib").join("librockchip_mpp.so"),
                rockchip_mpp_install_dir.join("lib").join("librockchip_vpu.so"),
            )
        )
    } else {
        (None, vec!())
    };

    let ffmpeg_out_dir = env_vars.out_dir.join("ffmpeg");
    let ffmpeg_install_dir = ffmpeg_out_dir.join("install");
    let mut ffmpeg_configure_cmd = Command::new(
        Path::new("vendor/ffmpeg/configure").canonicalize()
            .expect("ffmpeg configure absolute path")
    );
    ffmpeg_configure_cmd.current_dir("vendor/ffmpeg")
        .arg(format!("--prefix={ffmpeg_install_dir}"))
        .args([
            "--enable-gpl",
            "--enable-version3",
            "--disable-iconv",
            "--disable-zlib",
            "--disable-everything",
            "--disable-programs",
            "--disable-doc",
        ]);
    if let Some(ffmpeg_cross_opts) = ffmpeg_cross_opts {
        ffmpeg_configure_cmd
            .args(&ffmpeg_cross_opts);
    }
    if let Some(ref ffmpeg_pkg_config_path) = ffmpeg_pkg_config_path {
        // Detect if we are inside a nix shell
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
    }
    ffmpeg_configure_cmd.args(&env_vars.ffmpeg_configuration);
    assert!(
        ffmpeg_configure_cmd.status()
            .expect("Failed to run ffmpeg configuration")
            .success(),
        "Error configuring ffmpeg"
    );
    // FFMpeg produces object files just inside sources
    let ffmpeg_clean_status = Command::new("make")
        .args(["-C", "vendor/ffmpeg"])
        .arg("clean")
        .status()
        .expect("Failed to run ffmpeg cleaning");
    assert!(ffmpeg_clean_status.success(), "Error cleaning ffmpeg");
    let ffmpeg_build_status = Command::new("make")
        .args([
            "-C", "vendor/ffmpeg",
            "-j", &env_vars.num_jobs,
        ])
        .status()
        .expect("Failed to build ffmpeg");
    assert!(ffmpeg_build_status.success(), "Error building ffmpeg");
    let ffmpeg_install_status = Command::new("make")
        .args(["-C", "vendor/ffmpeg"])
        .arg("install")
        .status()
        .expect("Failed to run ffmpeg installation");
    assert!(ffmpeg_install_status.success(), "Error installing ffmpeg");

    for cleanup_file_path in &cleanup_files {
        // FIXME: Find out a way how to force a static linking
        fs::remove_file(cleanup_file_path)
            .expect(&format!("Failed to remove {cleanup_file_path} file"));
    }

    (
        ffmpeg_install_dir.join("include"),
        if let Some(ref ffmpeg_pkg_config_path) = ffmpeg_pkg_config_path {
            format!(
                "{}:{}",
                ffmpeg_pkg_config_path,
                ffmpeg_install_dir.join("lib").join("pkgconfig"),
            )
        } else {
            ffmpeg_install_dir.join("lib").join("pkgconfig").as_str().to_string()
        }
    )
}

fn main() {
    let env_vars = EnvVars::init();

    let (ffmpeg_include_dir, ffmpeg_pkg_config_path) = build_ffmpeg(&env_vars);

    linking(&env_vars, &ffmpeg_include_dir, &ffmpeg_pkg_config_path);
}
