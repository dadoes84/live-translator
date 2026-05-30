# 实时翻译器 · Live Translator

**实时屏幕翻译工具 — 框选屏幕区域，自动识别文字，AI 翻译为中文。**

A real-time on-screen translation tool — select any area of your screen, and it auto-detects text via OCR then translates to Chinese using AI.

[中文](#中文) | [English](#english)

---

## 中文

### 这是什么？

实时翻译器是一款 Windows 桌面工具。用户框选屏幕上的任意区域后，工具会自动截取该区域的图片，通过 Windows 内建 OCR 识别文字，并调用 DeepSeek AI 将文字翻译为简体中文。适用于游戏字幕、视频内嵌文字的实时翻译，无需手动复制粘贴。

### 功能特性

- **屏幕选区** — 拖拽框选任意屏幕区域，支持多显示器 + 不同 DPI 缩放
- **实时 OCR** — 调用 Windows 内置 OCR 引擎，支持英文、日文源语言识别
- **AI 翻译** — 使用 DeepSeek（deepseek-v4-flash / deepseek-v4-pro）模型，输出简体中文
- **智能稳定检测** — 检测到文字变化后等待内容稳定再触发翻译，避免「打字机式」逐字加载导致翻译不全
- **显示自定义** — 翻译区域支持自定义字体、字号、文字颜色、背景颜色
- **窗口置顶** — 一键将翻译窗口固定在最前，游戏/视频场景下始终可见
- **设置持久化** — 所有偏好设置（含 API Key）保存至本地，下次启动自动加载
- **隐私优先** — API Key 及所有设置项仅保存在本地，不收集、不上传任何信息。唯一的外部通讯是向 DeepSeek API 发送待翻译的文字
- **自定义提示词** — 可自由编辑 AI 的系统提示词，根据不同场景调整翻译风格、用词习惯，让译文更符合你的口味
- **全局快捷键** — 支持 `Ctrl+Shift+T` 开始/停止翻译、`Ctrl+Shift+A` 激活选区，可在设置中自定义

### 开发背景

玩各种 Galgame 时，机翻的质量实在让人难以忍受——生硬、不通顺，严重破坏代入感。于是决定自己动手做一个翻译器：用自己的 AI Prompt，调出对味的译文。从此不用再将就。

### 工作原理

```
框选区域 → 截图 → Windows OCR → 稳定检测 → DeepSeek AI → 显示译文
```

1. 点击「区域设置」，拖拽框选需要翻译的屏幕区域
2. 点击「开始」，启动翻译循环
3. 工具定期截取选区图片，通过 OCR 识别文字，发送至 DeepSeek 翻译
4. 译文实时显示在主界面中

### 系统要求

- **操作系统**：Windows 10（版本 1809+）或 Windows 11
- **OCR 语言包**：需安装英文和/或日文 OCR 组件
  - *设置 → 时间和语言 → 语言 → 添加语言 → 安装英文 / 日文的 OCR 组件*

### 安装

1. 从 [Releases](https://github.com/dadoes84/live-translator/releases) 下载最新版本
2. 解压后运行 `live-translator.exe`
3. 首次启动后，打开设置（齿轮图标），填入 DeepSeek API Key，点击「检测」验证连通性

### 使用步骤

| 步骤 | 操作 |
|------|------|
| 1 | 点击「区域设置」，拖拽框选待翻译的屏幕区域 |
| 2 | 根据源语言设置「OCR 语言」（英文/日文） |
| 3 | 点击齿轮图标进入设置，填入 DeepSeek API Key，选择模型，自定义提示词和快捷键 |
| 4 | 点击「开始」，启动实时翻译 |
| 5 | 点击 📌 将窗口置顶，方便边看边译 |

### 技术栈

- **Tauri v2** (Rust + WebView)
- **Vue 3** + TypeScript
- **Windows.Media.Ocr** (系统 OCR)
- **DeepSeek API** (AI 翻译)

---

## English

### What is this?

Live Translator is a Windows desktop app that captures a user-defined screen region, recognizes text via Windows OCR, and translates it into Chinese using the DeepSeek AI API. It's designed for translating game subtitles, video captions, or any on-screen text in real time — without needing to copy-paste anything.

### Features

- **Screen region selection** — drag to select any area across multiple monitors with full DPI awareness
- **Real-time OCR** — uses Windows built-in OCR engine, supports English and Japanese source text
- **AI translation** — powered by DeepSeek (deepseek-v4-flash / deepseek-v4-pro), outputs Chinese
- **Smart stability detection** — waits for text to stabilize before triggering translation, avoiding partial reads from "typewriter-style" captions
- **Customizable display** — font family, size, text color, and background color for the translation area
- **Always-on-top** — pin the window so translations stay visible above games or videos
- **Persistent settings** — all preferences (including API key) are saved locally and loaded on restart
- **Privacy first** — API key and all settings are stored locally on your machine. Nothing is uploaded, collected, or sent anywhere except to the DeepSeek API for translation
- **Custom prompts** — fully customizable system prompt for the AI, allowing you to tailor translation style and terminology for different contexts
- **Global shortcuts** — `Ctrl+Shift+T` to start/stop translation, `Ctrl+Shift+A` to activate area selection, customizable in settings

### Background

This tool was born out of frustration with the poor quality of machine translations in Japanese visual novels (Galgame). Off-the-shelf translators often produce stiff, unnatural results that break immersion. The solution: build your own translator with a customizable AI prompt, tuned specifically for the type of content you're reading.

### How It Works

```
Select Area → Screenshot → Windows OCR → Stability Check → DeepSeek AI → Display
```

1. Click **区域设置** (Area Setup) and drag to select the screen region containing text
2. Click **开始** (Start) to begin the capture loop
3. The app periodically screenshots the region, runs OCR, and sends recognized text to DeepSeek for translation
4. Translated text appears in the main display area

### System Requirements

- **OS**: Windows 10 (version 1809+) or Windows 11
- **OCR Language Packs**: English and/or Japanese OCR components must be installed
  - *Settings → Time & Language → Language → Add a language → install OCR for English / 日本語*

### Installation

1. Download the latest release from [Releases](https://github.com/dadoes84/live-translator/releases)
2. Extract and run `live-translator.exe`
3. On first launch, open Settings (gear icon), enter your DeepSeek API Key, and click **检测** (Test) to verify

### Usage

| Step | Action |
|------|--------|
| 1 | Click **区域设置** and drag to select the screen region to translate |
| 2 | Set **OCR 语言** to match the source text (English or Japanese) |
| 3 | Click the gear icon to open Settings, fill in your DeepSeek API Key, choose a model, customize prompt and shortcuts |
| 4 | Click **开始** to start real-time translation |
| 5 | Toggle **📌** (pin) to keep the window on top |

### Tech Stack

- **Tauri v2** (Rust + WebView)
- **Vue 3** + TypeScript
- **Windows.Media.Ocr** (system OCR)
- **DeepSeek API** (AI translation)

---

## Credits

本项目由 [Reasonix](https://github.com/esengine/DeepSeek-Reasonix) 辅助开发，翻译能力由 [DeepSeek](https://www.deepseek.com) 提供。

Built with [Reasonix](https://github.com/esengine/DeepSeek-Reasonix) · Powered by [DeepSeek](https://www.deepseek.com)
