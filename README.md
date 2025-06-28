

# Accessing Assests in Systems
Assets can be accessed in system using a special arugment called ```Asset<T>```
Once an asset is loaded with the asset_server.load("") method, the asset can then be accessed with the process 
outlined below

- Example
```rust
fn process_image_asset(images: Assets<Image>) {}
```

# React to Asset Changes with Asset Events
If you need to perform specific actions when an asset is created, modified, or removed, you can make a system that reacts to AssetEvent events.

- Example

```rust
fn handle_image_asset_creation(image_asset_event: EventReader<AssetEvent<Image>>) {}
```