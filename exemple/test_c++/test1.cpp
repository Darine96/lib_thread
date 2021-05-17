#include <pthread.h>
#include <stdio.h>

pthread_mutex_t mutex;
pthread_cond_t cond;

int buffer[100];

int loops = 5;
int length = 0;

void *producer(void *arg) {
    int i;
    for (i = 0; i < loops; i++) {
        pthread_mutex_lock(&mutex);
        buffer[length++] = i;
        printf("producer length %d\n", length);
        pthread_cond_signal(&cond);
        pthread_mutex_unlock(&mutex);
    }
}

void *consumer(void *arg) {
    int i;
    for (i = 0; i < loops; i++) {
        pthread_mutex_lock(&mutex);
        while(length == 0) {
            printf(" consumer waiting...\n");
            pthread_cond_wait(&cond, &mutex);
        }
        int item = buffer[--length];
        printf("Consumer %d\n", item);
        pthread_mutex_unlock(&mutex);
    }
}

int main(int argc, char *argv[])
{

    pthread_mutex_init(&mutex, 0);
    pthread_cond_init(&cond, 0);

    pthread_t pThread, cThread;
    pthread_create(&pThread, 0, producer, 0);
    pthread_create(&cThread, 0, consumer, 0);
    pthread_join(pThread, NULL);
    pthread_join(cThread, NULL);

    pthread_mutex_destroy(&mutex);
    pthread_cond_destroy(&cond);
    return 0;
}
