# Changelog

## [unreleased]
### Added
- `Context::make_texture` method

### Changed
...

### Fixed
...

## 0.0.1
- refactor: rename `Graphics::pixels_mut` to `Graphics::update_pixels`
- fix: No longer panic at the end of a program when `Context` is dropped
- refactor: optimize `Graphics::update_pixels`
- feat: Add `Graphics::pixels` method and `Pixels` type
- refactor: ([#1](https://github.com/pzipper/g2d/issues/1)) Make `Graphics` use a `&Texture` rather than a `&wgpu::Texture`
- feat: Add `Graphics::overwrite_pixel_data`