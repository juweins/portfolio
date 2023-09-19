# Progress outline and notes

In this document I will share my thoughts and progress on the project. I think this can be a good way to keep track of the changes I have made in the particular week. It may also outline, why particular changes have been made.

## 2023 KW 8
In this week I have been working on the following:

### 1. Splitting the lib.rs file into multiple files.

This is mainly because I want to move away from a single file. Given the project goal, dividing the code into producer / consumer modules is a good starting point to structure the growing codebase.
The azure specific functions will be migrated to the new files as well.

### 2. Expanded the api_key functions to support multiple api keys for upcoming features.

I plan on integrating another api to do some simple data manipulation in Azure data factory. This will require a second api to be integrated.

### 3. Separated the keys and also added invalid keys to outline the used keys and prepare for upcoming integration/unit tests.

It is a good practise to test the code with invalid keys. There should be a meaningful error message when the key is invalid. This is a good way to test the error handling of the code.

### 4. Enhanced the code coverage by adding unit & integration tests.

Testing the consumer is a bit tricky, because it is a blocking function. I have to find a way to test this in a proper way. I moved the consumer in a separate thread to be able to test the consumer. Now everything is working as expected. Quite a steep learning curve, I am happy with the result.

### 5. Implemented the kafka consumer.

This first version is a simple consumer based on BaseConsumer which allows custom polling. This was kind of a struggle, because testing the conumer is not that easy since it is a blocking function. For my intended implementation I have to alter the original use case for a kafka consumer. I overcome this by using a idle check to see if there are any messages in the queue. The consumer shuts down when there are no more messages for x subsequent polls. This is not the best solution, but it works for now.

### 6. Wrapped the application in a clap CLI app.

Today (02/18) I finally wrapped the application in a first version of an CLI app. This will be the main entry point and the targeted use case for the application. As described in the project outline, the binary can easily be deployed to a docker container or a VM. From this point on, it will just be a matter of developing a sequence of commands to be executed one after the other.

<br></br>

## 2023 KW 9 🎉 **1000 lines of code** 🎉

My project just crossed the 1000 lines of code mark. I am happy with the progress so far. In this week I have been working on the following:

### 1. Finished the implementation of argument parsing.
Open todo: Find a way to make the arguments dynamically for supporting multiple api's. This is not a priority at the moment, but it would be nice to have. I will probably have to use a macro to achieve this. For now, every api requested will need the same arguments - even if they are not used/required.

### 2. Enabled the flexible use for multiple api.
For this I had to change the function signature of the request function. Instead of using an explicit api structure (Exchange) I chose to return a generic json. This way I can use the same function for multiple api's. I still have to figure out how users may dynamically add new api's to the application, since the exchangerates api need a string literal (as of now). This problem may be solved by itself when I switch to exchangerates_api/latest. It may limit api configurability. Anyway, this will be adressed in a future feature.

### 3. Implemented the azure read function. (& extended the CLI app)
This function reads the data from the azure blob storage as defined in the config file. I will probably need the function in a future feature, for now it is just implemented for completeness.

## 2023 KW 10
This week I have been working on the following:

### 1. Finalized the azure write function

### 2. Reorganized the codebase
In order to keep the project maintainable, I decided to move azure functions into a dedicated directory. By doing so, I also moved the kafka files into theirs. This way I can keep the codebase clean and easy to navigate.

### 3. Implemented the Config command
First, I wanted to implement get/set config functions to edit each config precisely. However, I found it more convenient (and easier to implement) to just edit the config file directly via nano/vim. This way I can also use the config file as a template for the user. The config command therefore just opens the file and lets the user edit it. This is a good compromise between usability and maintainability.

### 4. Set up the terraform infrastructure
Despite the original plan I decided to leverage the true strength of rust and use a ephemeral azure function (aws lambda complement) to run the application. This way I can easily deploy the application to the cloud and run it as a service. I will probably use the azure function to run directly. Another way, that I am investigation at the moment is putting the application in a docker container.
Deploying the binary on a dedicated VM is kind of a overkill, since the application is not that compute intensive - the costs would also be too high.