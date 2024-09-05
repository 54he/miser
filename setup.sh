#!/bin/bash
rustup default nightly
cargo build --release
mkdir /server &&mkdir -p /var/log/minser&&touch /var/log/minser/connect.log
cp target/release/miser /bin
echo "miser已经安装到bin"
cat <<EOF> /server/index.html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
EOF
cat <<EOF> /server/404.html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>404</title>
  </head>
  <body>
    <h1>SERVER</h1>
    <p>NOT FOUND</p>
  </body>
</html>
EOF

