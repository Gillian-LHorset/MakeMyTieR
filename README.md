# MakeMyTieR

![MakeMyTieR Logo](MakeMyTieR-Logo.png)

---

MakeMyTieR est une application de création de tier lists faite en Rust avec Egui.

## Installation

### 1. Installe MSYS2, qui fournit l'environnement MinGW :
```bash
winget install MSYS2.MSYS2
```

### 2. Ouvre MSYS2 (menu Démarrer) et installe le compilateur et les outils :

```bash
pacman -Syu
pacman -S mingw-w64-x86_64-toolchain
```

### 3. Ajoute ce répertoire au PATH Windows. 
Paramètres → Variables d'environnement → Path → Nouveau : ``C:\msys64\mingw64\bin``.

### 4. Installe rustup :

```bash
winget install Rustlang.Rustup
```

### 5. Installe le toolchain GNU et mets-le par défaut :

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
rustup set default-host x86_64-pc-windows-gnu
```

### 6. Vérifie :
```bash
rustc -vV
```
Le host: de rustc doit afficher x86_64-pc-windows-gnu.

### 7. Teste avec ton projet :
```bash
cd C:\Users\pq60soi\Desktop\MakeMyTieR\MakeMyTieR
cargo run
```
