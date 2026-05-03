use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sched::{unshare, CloneFlags};
use nix::unistd::{chdir, pivot_root};
use std::env;
use std::fs;
use std::process::{self, Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--child" {
        run_neo();
    } else {
        run_operator();
    }
}

fn run_operator() {
    let base_dir = "/tmp/snap-shell";
    let (upper, work, merged) = (format!("{}/upper", base_dir), format!("{}/work", base_dir), format!("{}/merged", base_dir));

    println!("[*] [Opérateur] Initialisation de l'environnement sécurisé...");
    for dir in [&upper, &work, &merged] { let _ = fs::create_dir_all(dir); }

    let current_exe = env::current_exe().unwrap();
    let mut child = Command::new(current_exe)
        .arg("--child")
        .spawn()
        .expect("Erreur au lancement de Neo");

    child.wait().unwrap();

    println!("[*] [Opérateur] Nettoyage des traces...");
    let _ = fs::remove_dir_all(base_dir);
    println!("[+] Terminé.");
}

fn run_neo() {
    let base_dir = "/tmp/snap-shell";
    let merged = format!("{}/merged", base_dir);
    let put_old = format!("{}/old_root", merged);

    // --- ISOLATION RÉSEAU + MONTAGES ---
    // CLONE_NEWNET coupe la connexion internet dans la bulle
    let _ = unshare(CloneFlags::CLONE_NEWNS | CloneFlags::CLONE_NEWNET);
    let _ = mount(None::<&str>, "/", None::<&str>, MsFlags::MS_PRIVATE | MsFlags::MS_REC, None::<&str>);

    // Montage OverlayFS
    let mount_data = format!("lowerdir=/,upperdir={0}/upper,workdir={0}/work", base_dir);
    let _ = mount(Some("overlay"), merged.as_str(), Some("overlay"), MsFlags::empty(), Some(mount_data.as_str()));

    // Bind Mounts vitaux
    for sys in &["proc", "dev", "sys"] {
        let target = format!("{}/{}", merged, sys);
        let _ = mount(Some(format!("/{}", sys).as_str()), target.as_str(), None::<&str>, MsFlags::MS_BIND, None::<&str>);
    }

    // Pivot Root
    let _ = fs::create_dir_all(&put_old);
    let _ = mount(Some(merged.as_str()), merged.as_str(), None::<&str>, MsFlags::MS_BIND, None::<&str>);
    pivot_root(merged.as_str(), put_old.as_str()).unwrap();

    let _ = chdir("/");
    let _ = umount2("/old_root", MntFlags::MNT_DETACH);
    let _ = fs::remove_dir("/old_root");

    println!("[+] [Matrice] Réseau coupé. Isolation totale.");
    Command::new("/bin/bash").spawn().unwrap().wait().unwrap();
}