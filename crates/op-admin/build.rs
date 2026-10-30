fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../assets/branding/open-presenter-app-icon.ico");
        res.set("ProductName", "OpenPresenter");
        res.set(
            "FileDescription",
            "OpenPresenter Minimal - Projeção para igrejas e eventos",
        );
        res.set("LegalCopyright", "GPL-3.0-only");
        if let Err(e) = res.compile() {
            eprintln!("Aviso: Falha ao compilar recursos do Windows: {}", e);
        }
    }
}
