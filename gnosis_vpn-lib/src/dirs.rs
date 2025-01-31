use directories::ProjectDirs;

pub fn project() -> Option<ProjectDirs> {
    ProjectDirs::from("org", "hoprnet", "gnosisvpn")
}
