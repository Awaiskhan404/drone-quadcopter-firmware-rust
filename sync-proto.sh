rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh proto/ $RPI_ADDR:/home/pi/$DRONE_FOLDER/proto
rsync -v -r --exclude '*target/*' --exclude '.git/' --exclude '.idea/' -e ssh protos/src $RPI_ADDR:/home/pi/$DRONE_FOLDER/protos/src


