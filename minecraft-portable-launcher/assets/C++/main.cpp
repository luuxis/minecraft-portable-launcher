#include <iostream>

int download(std::string url, std::string destination) {
    std::cout << "telechargement de minecraft\n";
    system(("if not exist " + destination + " powershell -Command \"(New-Object Net.WebClient).DownloadFile('" + url + "', '" + destination + "')\" ").c_str());
    return 0;
}


int main() {
    std::cout << "Programme d'installation\n";
    system("title minecraft");
    system("if not exist %CD%\\Minecraft\\AppData\\.minecraft md %CD%\\Minecraft\\AppData\\.minecraft");
    system("if not exist %CD%\\Minecraft\\Minecraft mkdir %CD%\\Minecraft\\Minecraft");
    download("https://launcher.mojang.com/download/Minecraft.exe", "Minecraft\\Minecraft\\minecraft.exe");
    system("start %CD%\\Minecraft\\Minecraft\\Minecraft.exe --workDir %CD%\\Minecraft\\AppData\\.minecraft");
    return 0;
}