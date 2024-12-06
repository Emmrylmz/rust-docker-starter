# Rust Starter Projesi 🦀🐳

## 📋 Proje Açıklaması

Bu proje, Rust kullanarak hızlı bir başlangıç yapmak isteyenler için hazırlanmış örnek bir projedir. Docker ve Docker Compose ile entegre edilmiştir.

## 🛠 Gereksinimler

Projeyi çalıştırmadan önce aşağıdaki yazılımların sisteminizde kurulu olduğundan emin olun:

- Docker
- Docker Compose
- Git

## 🚀 Kurulum ve Başlangıç

### 1. Depoyu Klonlama

```bash
git clone https://github.com/Emmrylmz/rust-docker-starter.git

# Proje dizinine girme
cd <repository-folder>
```

### 2. Docker Compose ile Çalıştırma

```bash
# Projeyi ayağa kaldırma
docker-compose up --build

# Arka planda çalıştırma
docker-compose up -d --build
```

## 🔍 Kodları İzleme ve Test Çalıştırma

`cargo-watch` container içinde otomatik olarak kaynak dosyalarını izler ve testleri çalıştırır. Kodda bir değişiklik yaptığınızda otomatik olarak yeniden çalışacaktır.

### Testleri Çalıştırma

```bash
docker-compose run app cargo test
```

### Hizmeti Durdurma

```bash
docker-compose down
```

## 📂 Proje Yapısı

```
proje-dizini/
│
├── src/
│   └── main.rs         # Ana Rust kaynak dosyası
│
├── Dockerfile          # Docker yapılandırma dosyası
├── docker-compose.yml  # Docker Compose yapılandırması
└── README.md           # Proje dökümantasyonu
```

### Dosya İşlevleri

- `src/main.rs`: Rust projesinin ana dosyası. Basit bir "Hello World!" uygulamasını içerir.
- `Dockerfile`: Rust projesini çalıştırmak için Docker yapılandırma dosyası.
- `docker-compose.yml`: Çoklu container yönetimi için Compose yapılandırma dosyası.

## 🛠 Kullanılan Araçlar

- **Rust**: Alpine tabanlı Docker imajı ile hafif bir Rust çalışma ortamı
- **cargo-watch**: Kod değişikliklerini izlemek ve otomatik test çalıştırmak için kullanılır

