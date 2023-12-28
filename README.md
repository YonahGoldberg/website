# My personal Website!

# Build and Deploy Commands:
1. docker buildx build --load --platform=linux/amd64 -t website .
2. docker tag website:latest 946283733563.dkr.ecr.us-east-1.amazonaws.com/website
3. docker push 946283733563.dkr.ecr.us-east-1.amazonaws.com/website