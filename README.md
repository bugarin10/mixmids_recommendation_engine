# MB_Owner_Login
Serverless Rust Microservice

To make your application a serverless Rust microservice, you can leverage a framework like AWS Lambda with API Gateway to host and serve your application. Here's a high-level overview of the steps involved:

1. Convert HTML/CSS/JS to a Rust Web Application: Rewrite your front-end code in Rust using a web framework like Rocket or Actix-web. You'll need to use Rust crates for serving HTML/CSS/JS files and handling HTTP requests.
2. Deploy as an AWS Lambda Function: Package your Rust application as a Lambda function. You'll need to create a deployment package containing your Rust executable and any dependencies. AWS Lambda supports running custom runtimes, so you can use a Docker container to build your Rust executable and include it in the deployment package.
3. Set Up API Gateway: Create an API Gateway to expose your Lambda function over HTTP. Configure the API Gateway to trigger your Lambda function when receiving HTTP requests. You can define routes and methods to map HTTP requests to specific Lambda functions.
4. Handle Static Assets: Serve your static assets (HTML/CSS/JS files) either directly from AWS Lambda or using AWS services like S3 for storage and CloudFront for content delivery. You'll need to configure your Rust application to serve these assets when requested by the client.
5. Test and Monitor: Test your serverless application thoroughly to ensure it behaves as expected. Use AWS CloudWatch to monitor the performance and logs of your Lambda function and API Gateway.
6. Set Up CI/CD: Implement a CI/CD pipeline to automate the deployment of your serverless application. You can use AWS CodePipeline, GitHub Actions, or other CI/CD tools to build, test, and deploy your application whenever you push changes to your repository.

```bash
docker build -t <USERNAME>/my-rust-lambda .
```

Note: Remember to change your Dockerfile to 
    
```Dockerfile
WORKDIR /usr/src/<YOUR PACKAGE NAME FROM Cargo.toml>
```

# Setting up DynamoDB
1. Create a table in DynamoDB
+ Table Name: Something specific to your application, e.g., "User_Login"
+ Primary Key: This will help you to uniquely identify each item in the table. You can use a single attribute as the primary key (e.g., "username") or a combination of attributes (e.g., "username" and "Timestamp"). 
+ Sort Key: If you choose a composite primary key, you can add a sort key to further refine the primary key. This will help you to sort the items in the table based on the sort key attribute. 

2. Create an IAM Role for Lambda. Similar to the previous IAM users created, the current IAM user has the following permissions:
+ AmazonAPIGatewayInvokeFullAccess	
+ AmazonDynamoDBFullAccess	
+ AWSLambda_FullAccess	
+ AWSLambdaDynamoDBExecutionRole
+ AWSLambdaInvocation-DynamoDB	
+ CloudWatchLambdaInsightsExecutionRolePolicy	
+ CloudWatchLogsFullAccess

3. Configure you IAM role using `aws configure` command in your terminal. This will set up your environment to test your lambda function locally.