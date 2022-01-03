#!/bin/bash

if [[ -z "${GITHUB_TOKEN}" ]]
then
  echo "ERROR: \$GITHUB_TOKEN was unset or blank."
  exit 1
fi

if [[ -z "${GITHUB_USERNAME}" ]]
then
  echo "ERROR: \$GITHUB_USERNAME was unset or blank."
  exit 1
fi

echo "Logging into GitHub container repository..."

echo ${GITHUB_TOKEN} \
  | docker login ghcr.io \
    -u ${GITHUB_USERNAME} \
    --password-stdin \
|| exit 1

imageTag="ghcr.io/${GITHUB_USERNAME}/battlesnake"

echo "Building image..."

docker build \
  -t ${imageTag} \
  .. \
|| exit 1

echo "Pushing image..."

docker push \
  ${imageTag}:latest \
|| exit 1
