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
