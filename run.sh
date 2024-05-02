echo "---------------- CLEANING PREVIOUS ----------------"

rm ./output/test

echo "---------------- BUILDING ----------------"

python build.py

echo "---------------- RUNNING ----------------"

LD_LIBRARY_PATH=output/libs output/test

echo "---------------- DONE ----------------"