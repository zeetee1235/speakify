# Speakify

이미지를 스핔이로 변환하는 도구

사용: https://zeetee1235.github.io/speakify/

![Example](네르_Cuayo.gif)

## 빌드

```bash
cargo build --release
```

## 사용법 (linux)

### 쉘 스크립트

```bash
./speakify.sh <화질> <입력이미지>
```

**화질 옵션**:
- `low` - 64x64, 50프레임 (~1초)
- `mid` - 128x128, 100프레임 (~5초)
- `high` - 256x256, 150프레임 (~30초)

**예시**:
```bash
./speakify.sh low photo.jpg      # 빠른 미리보기
./speakify.sh mid image.png      # 기본 품질
./speakify.sh high portrait.webp # 고품질
```

출력: `(입력파일명)_Cuayo.gif`

### 직접 실행

```bash
./target/release/speakify -i <입력> -r <해상도> -f <프레임수>
```

## 지원 포맷

입력: PNG, JPEG, WebP  
출력: GIF

## 기여

언제나 대환영

## 크레딧

이 프로젝트는 [obamify](https://github.com/Spu7Nix/obamify)에서 영감을 받았습니다.

## 라이선스

MIT
