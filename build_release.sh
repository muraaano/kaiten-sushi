MACOSX_DEPLOYMENT_TARGET=10.15
TARGET=x86_64-apple-darwin
# TODO please change tag version everytime
tag="v0.1.0"
name="kaiten-sushi-$tag-$TARGET"

cargo build --release --target $TARGET
mkdir $name
cp target/$TARGET/release/kaiten-sushi $name/
cd $name
tar czf ../$name.tar.gz *
cd ..
rm -rf $name

echo "upload '$name.tar.gz' to GitHub release. https://github.com/Samemura/kaiten-sushi/releases"