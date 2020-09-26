# Browser selector
Just a hello world for Rust

The app allows opening different links in different browser

The app registers as browser by default and proxies the calls to various browser depending on URLs

## Usage
```CMD
REM register the app as default browser
> startprototype.exe register

REM remove the app from registry browser list
> startprototype.exe unregister
```

## Configuration
```toml
default = "C:\\Program Files\\Mozilla Firefox\\firefox.exe"

[browsers.ie]
path = "C:\\Program Files\\Internet Explorer\\iexplore.exe"
words = [
    "yandex.com.tr",
    "myword"
]
```
If any of the `words` met in opening URL the link will be opened in specified browser. Otherwise, the link opens in default browser