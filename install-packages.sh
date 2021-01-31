curl -sL https://deb.nodesource.com/setup_12.x | sudo bash
sudo apt-get install -y nodejs

sudo snap install nvim

curl -fLo ~/.vim/autoload/plug.vim --create-dirs \
    https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim