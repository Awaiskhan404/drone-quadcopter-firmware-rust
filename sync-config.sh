rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh config.json $RPI_ADDR:/home/pi/$DRONE_FOLDER/config.json