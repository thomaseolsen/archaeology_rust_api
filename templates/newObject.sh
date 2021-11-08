#!/bin/sh

# This script provides an automated process to create scaffolds for new objects within this project.

echo "What is the proper name of the new object (e.g.: Site, Square, Locus, etc.)?"
read OBJECT
echo "What is the lower-case name of the new object (e.g.: site, square, locus, etc.)?"
read OBJECT_LOWER

echo "Creating $OBJECT model."
NEWFILE="src/models/"
NEWFILE+=$OBJECT_LOWER
NEWFILE+="_model.rs"
cp templates/object/new_model.rs $NEWFILE
echo "Model creation completed with exit status: $?"
NEWMODEL+=$OBJECT_LOWER
NEWMODEL+="_model;"
echo "pub mod $NEWMODEL" >> src/models.rs

echo "Creating $OBJECT service."
NEWFILE="src/services/"
NEWFILE+=$OBJECT_LOWER
NEWFILE+="_service.rs"
cp templates/object/new_service.rs $NEWFILE
echo "Service creation completed with exit status: $?"
NEWSERVICE+=$OBJECT_LOWER
NEWSERVICE+="_service;"
echo "pub mod $NEWSERVICE" >> src/services.rs
