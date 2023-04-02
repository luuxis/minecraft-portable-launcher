#include <iostream>

int download(std::string url, std::string destination) {
    std::cout << "telechargement de minecraft\n";
    system(("if not exist " + destination + " powershell -Command \"(New-Object Net.WebClient).DownloadFile('" + url + "', '" + destination + "')\" ").c_str());
    return 0;
}


int main() {
    std::cout << "Programme d'installation\n";
    system("title Minecraft");
    system("if not exist \"%CD%\\Minecraft\\.minecraft\" mkdir \"%CD%\\Minecraft\\.minecraft\"");
    system("if not exist \"%CD%\\Minecraft\\Minecraft Launcher\" mkdir \"%CD%\\Minecraft\\Minecraft Launcher\"");
    download("https://launcher.mojang.com/download/Minecraft.exe", "\"%CD%\\Minecraft\\Minecraft Launcher\\Minecraft.exe\"");
    system("start \"\" \"%CD%\\Minecraft\\Minecraft Launcher\\Minecraft.exe\" --workDir \"%CD%\\Minecraft\\.minecraft\" --tmpDir \"%CD%\\Minecraft\\Minecraft Launcher\" --user-data-dir \"%CD%\\Minecraft\\data user\"");
    return 0;
}
