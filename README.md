

# Accessing Assests in Systems
Assets can be accessed in system using a special arugment called ```Asset<T>```
Once an asset is loaded with the asset_server.load("") method, the asset can then be accessed with the process 
outlined below

- Example
```rust
fn process_image_asset(images: Assets<Image>) {}
```