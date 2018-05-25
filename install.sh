#! /bin/bash

URL=$(curl https://api.github.com/repos/kogai/trs/releases/latest | grep 'browser_download_url' | grep -i "$(uname)" | cut -d\" -f4)
FILE=$(echo $URL | sed 's/.*\(trs-.*\).tar.gz/\1/')

curl -L "$URL" > "/tmp/$FILE.tar.gz"
mkdir -p "/tmp/$FILE"
tar -xvf "/tmp/$FILE.tar.gz" -C "/tmp/$FILE"
mkdir -p ~/bin
mv "/tmp/$FILE/bin/$(uname)/trs" ~/bin
