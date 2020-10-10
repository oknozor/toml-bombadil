#!/usr/bin/env bash
set -euo pipefail

TEMP_FILE=/tmp/bombadil_man.md
DATE=$(date +"%dth %B, %Y")

while getopts v:i:o: option
do
case "${option}"
in
v) VERSION=${OPTARG};;
i) INPUT_PATH=${OPTARG};;
o) OUTPUT_PATH=${OPTARG};;
*) echo "invalid flag"
esac
done

echo -e "Creating temporary file"
cp "$INPUT_PATH" "$TEMP_FILE"

echo -e "Setting version to $VERSION"
sed -i -e "s/{{version}}/$VERSION/g" "$TEMP_FILE"
echo -e "Setting date released to $DATE"
sed -i -e "s/{{date_released}}/$DATE/g" "$TEMP_FILE"

pandoc "$TEMP_FILE" -s -t man > "$OUTPUT_PATH"
