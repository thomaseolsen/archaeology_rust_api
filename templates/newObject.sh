#!/bin/sh

# This script provides an automated process to create scaffolds for new objects within this project.

echo "What is the name of the new object (e.g.: site, square, locus, etc.)?"
read -r OBJECT

echo "Creating $OBJECT model."
NEWFILE="src/models/"
NEWFILE=$NEWFILE$OBJECT
NEWFILE=$NEWFILE"_model.rs"
cp templates/object/new_model.rs "$NEWFILE"
echo "Model creation completed with exit status: $?"
NEWMODEL=$OBJECT
NEWMODEL=$NEWMODEL"_model;"
echo "pub mod $NEWMODEL" >> src/models.rs

echo "Creating $OBJECT service."
NEWFILE="src/services/"
NEWFILE=$NEWFILE$OBJECT
NEWFILE=$NEWFILE"_service.rs"
cp templates/object/new_service.rs "$NEWFILE"
echo "Service creation completed with exit status: $?"
NEWSERVICE=$OBJECT
NEWSERVICE=$NEWSERVICE"_service;"
echo "pub mod $NEWSERVICE" >> src/services.rs
