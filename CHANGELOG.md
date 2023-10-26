# Changelog

## [unreleased]
- fix: No longer panic at the end of a program when `Context` is dropped
- refactor: optimize `Graphics::pixels_mut`
- feat: Add `Graphics::pixels` method and `Pixels` type
- refactor: ([#1](https://github.com/pzipper/g2d/issues/1)) Make `Graphics` use a `&Texture` rather than a `&wgpu::Texture`