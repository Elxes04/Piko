use std::collections::HashMap;
use std::fs;
use std::path::Path;
// use anyhow::Result;

pub struct DistroLogo {
    #[allow(dead_code)]
    pub name: String,
    pub ascii_art: Vec<String>,
    #[allow(dead_code)]
    pub colors: Vec<String>,
}

impl DistroLogo {
    pub fn new(name: String, ascii_art: Vec<String>, colors: Vec<String>) -> Self {
        DistroLogo {
            name,
            ascii_art,
            colors,
        }
    }
}

pub struct LogoManager {
    logos: HashMap<String, DistroLogo>,
}

impl LogoManager {
    pub fn new() -> Self {
        let mut manager = LogoManager {
            logos: HashMap::new(),
        };
        manager.load_builtin_logos();
        manager
    }

    fn load_builtin_logos(&mut self) {
        // Arch Linux
        self.logos.insert("arch".to_string(), DistroLogo::new(
            "Arch Linux".to_string(),
            vec![
                "                   -`".to_string(),
                "                  .o+`".to_string(),
                "                 `ooo/".to_string(),
                "                `+oooo:".to_string(),
                "               `+oooooo:".to_string(),
                "               -+oooooo+:".to_string(),
                "             `/:-:++oooo+:".to_string(),
                "            `/++++/+++++++:".to_string(),
                "           `/++++++++++++++:".to_string(),
                "          `/+++ooooooooooooo/`".to_string(),
                "         ./ooosssso++osssssso+`".to_string(),
                "        .oossssso-````/ossssss+`".to_string(),
                "       -osssssso.      :ssssssso.".to_string(),
                "      :osssssss/        osssso+++.".to_string(),
                "     /ossssssss/        +ssssooo/-".to_string(),
                "   `/ossssso+/:-        -:/+osssso+-".to_string(),
                "  `+sso+:-`                 `.-/+oso:".to_string(),
                " `++:.                           `-/+/".to_string(),
                " .`                                 `/".to_string(),
            ],
            vec![
                "#1793D1".to_string(), // Arch blue
            ],
        ));

        // Ubuntu
        self.logos.insert("ubuntu".to_string(), DistroLogo::new(
            "Ubuntu".to_string(),
            vec![
                "                          ./+o+-".to_string(),
                "                  yyyyy- -yyyyyy+".to_string(),
                "               ://+//////-yyyyyyo".to_string(),
                "           .++ .:/++++++/-.+sss/`".to_string(),
                "         .:++o:  /++++++++/:--:/-".to_string(),
                "        o:+o+:++.`..```.-/oo+++++/".to_string(),
                "       .:+o:+o/.          `+sssoo+/".to_string(),
                "  .++/+:+oo+o:`             /sssooo.".to_string(),
                " /+++//+:`oo+o               /::--:.".to_string(),
                r" \+/+o+++`o++o               ++////.".to_string(),
                "  .++.o+++oo+:`             /dddhhh.".to_string(),
                "       .+.o+oo:.          `oddhhhh+".to_string(),
                r"        \+.++o+o``-````.:ohdhhhhh+".to_string(),
                "         `:o+++ `ohhhhhhhhyo++os:".to_string(),
                "           .o:`.syhhhhhhh/.oo++o`".to_string(),
                "               /osyyyyyyo++ooo+++/".to_string(),
                r"                   ````` +oo+++o\:".to_string(),
                "                          `oo++.".to_string(),
            ],
            vec![
                "#E95420".to_string(), // Ubuntu orange
                "#772953".to_string(), // Ubuntu purple
            ],
        ));

        // Debian
        self.logos.insert("debian".to_string(), DistroLogo::new(
            "Debian".to_string(),
            vec![
                "  _____".to_string(),
                " /  ___|".to_string(),
                "|  |  __".to_string(),
                "|  | |_ |".to_string(),
                "|  |__| |".to_string(),
                r" \_____|".to_string(),
            ],
            vec![
                "#D70A53".to_string(), // Debian red
            ],
        ));

        // Fedora
        self.logos.insert("fedora".to_string(), DistroLogo::new(
            "Fedora".to_string(),
            vec![
                "             .',;::::;,'.".to_string(),
                "         .';:cccccccccccc:;:.".to_string(),
                "      .;cccccccccccccccccccccc;.".to_string(),
                "    .:cccccccccccccccccccccccccc:.".to_string(),
                "  .;ccccccccccccc;.:dddl:.;ccccccc;.".to_string(),
                " .:ccccccccccccc;OWMKOOXMWd;ccccccc:.".to_string(),
                ".:ccccccccccccc;KMMc;cc;xMMc:ccccccc:.".to_string(),
                ",cccccccccccccc;MMM.;cc;;WW::cccccccc,".to_string(),
                ":cccccccccccccc;MMM.;cccccccccccccccc:".to_string(),
                ":ccccccc;oxOOOo;MMM0OOkxo;cccccccccccc:".to_string(),
                ",cccccccccccccc;MMM.;cccccccccccccccc,".to_string(),
                ".:cccccccccccccc;MMMc.;ccccccccccccccc:.".to_string(),
                " .:cccccccccccccc;ddddl:.;cccccccccccc:.".to_string(),
                "  .;cccccccccccccccccccccccccccccc;.".to_string(),
                "    .:cccccccccccccccccccccccccc:.".to_string(),
                "      .;cccccccccccccccccccccc;.".to_string(),
                "         .';:cccccccccccc:;:.".to_string(),
                "             .',;::::;,'.".to_string(),
            ],
            vec![
                "#0DB7ED".to_string(), // Fedora blue
                "#294172".to_string(), // Fedora dark blue
            ],
        ));

        // Manjaro
        self.logos.insert("manjaro".to_string(), DistroLogo::new(
            "Manjaro".to_string(),
            vec![
                "██████████████████  ████████".to_string(),
                "██████████████████  ████████".to_string(),
                "██████████████████  ████████".to_string(),
                "██████████████████  ████████".to_string(),
                "████████            ████████".to_string(),
                "████████  ████████  ████████".to_string(),
                "████████  ████████  ████████".to_string(),
                "████████  ████████  ████████".to_string(),
                "████████  ████████  ████████".to_string(),
            ],
            vec![
                "#35BF5C".to_string(), // Manjaro green
            ],
        ));

        // OpenSUSE
        self.logos.insert("opensuse".to_string(), DistroLogo::new(
            "openSUSE".to_string(),
            vec![
                "  ______".to_string(),
                " / ____/".to_string(),
                "/ /     ".to_string(),
                "/ /___  ".to_string(),
                "\\____/  ".to_string(),
            ],
            vec![
                "#73BA25".to_string(), // OpenSUSE green
            ],
        ));

        // CentOS
        self.logos.insert("centos".to_string(), DistroLogo::new(
            "CentOS".to_string(),
            vec![
                " ____".to_string(),
                "/ ___|".to_string(),
                "\\___ \\".to_string(),
                " ___) |".to_string(),
                "|____/".to_string(),
            ],
            vec![
                "#932279".to_string(), // CentOS purple
            ],
        ));

        // Alpine
        self.logos.insert("alpine".to_string(), DistroLogo::new(
            "Alpine".to_string(),
            vec![
                "   /\\".to_string(),
                "  /  \\".to_string(),
                " /    \\".to_string(),
                "/______\\".to_string(),
            ],
            vec![
                "#0D597F".to_string(), // Alpine blue
            ],
        ));

        // Gentoo
        self.logos.insert("gentoo".to_string(), DistroLogo::new(
            "Gentoo".to_string(),
            vec![
                " _-----_".to_string(),
                "(       \\".to_string(),
                "\\    0   \\".to_string(),
                " \\        )".to_string(),
                " /      _/".to_string(),
                "(     _-".to_string(),
                "\\____-".to_string(),
            ],
            vec![
                "#54487A".to_string(), // Gentoo purple
            ],
        ));

        // Default/Unknown
        self.logos.insert("default".to_string(), DistroLogo::new(
            "Linux".to_string(),
            vec![
                "    __  ___".to_string(),
                "   /  |/  /".to_string(),
                "  / /|_/ /".to_string(),
                " / /  / /".to_string(),
                "/_/  /_/".to_string(),
            ],
            vec![
                "#000000".to_string(), // Black
            ],
        ));
    }

    pub fn detect_distro() -> String {
        // Try to detect the distribution
        if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
            for line in os_release.lines() {
                if line.starts_with("ID=") {
                    let id = line[3..].trim_matches('"');
                    return id.to_lowercase();
                }
            }
        }

        // Fallback detection methods
        if Path::new("/etc/arch-release").exists() {
            return "arch".to_string();
        }
        if Path::new("/etc/debian_version").exists() {
            return "debian".to_string();
        }
        if Path::new("/etc/fedora-release").exists() {
            return "fedora".to_string();
        }
        if Path::new("/etc/redhat-release").exists() {
            return "centos".to_string();
        }

        "default".to_string()
    }

    pub fn get_logo(&self, distro: &str) -> Option<&DistroLogo> {
        self.logos.get(distro)
    }

    pub fn get_detected_logo(&self) -> &DistroLogo {
        let detected = Self::detect_distro();
        self.get_logo(&detected).unwrap_or_else(|| self.get_logo("default").unwrap())
    }



    pub fn list_available_logos(&self) -> Vec<&String> {
        self.logos.keys().collect()
    }
}
