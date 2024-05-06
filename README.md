# My personal Website!

# Build and Deploy Commands:
1. aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin 946283733563.dkr.ecr.us-east-1.amazonaws.com
2. docker buildx build --load --platform=linux/amd64 -t website .
3. docker tag website:latest 946283733563.dkr.ecr.us-east-1.amazonaws.com/website
4. docker push 946283733563.dkr.ecr.us-east-1.amazonaws.com/website