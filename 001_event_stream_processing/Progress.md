# Progress outline and notes

In this document I will share my thoughts and progress on the project. I think this can be a good way to keep track of the changes I have made in the particular week. It may also outline, why particular changes have been made.

## 2023 KW 8
In this week I have been working on the following:

### 1. Splitting the lib.rs file into multiple files.

This is mainly because I want to move away from a single file. Given the project goal, dividing the code into producer / consumer modules is a good starting point to structure the growing codebase.
The azure specific functions will be migrated to the new files as well.

### 2. Expanded the api_key functions to support multiple api keys for upcoming features.

### 3. Separated the keys and also added invalid keys to outline the used keys and prepare for upcoming integration/unit tests.

### 4. Enhanced the code coverage by adding unit tests.

### 5. Implemented the kafka consumer.
This first version is a simple consumer based on BaseConsumer which allows custom polling. This was kind of a struggle, because testing the conumer is not that easy since it is a blocking function. For my intended implementation I have to alter the original use case for a kafka consumer. I overcome this by using a idle check to see if there are any messages in the queue. The consumer shuts down when there are no more messages for x subsequent polls. This is not the best solution, but it works for now.
