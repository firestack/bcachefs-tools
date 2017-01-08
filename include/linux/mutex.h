#ifndef __TOOLS_LINUX_MUTEX_H
#define __TOOLS_LINUX_MUTEX_H

#include <pthread.h>

struct mutex {
	pthread_mutex_t lock;
};

#define mutex_init(l)		pthread_mutex_init(&(l)->lock, NULL)
#define mutex_lock(l)		pthread_mutex_lock(&(l)->lock)
#define mutex_trylock(l)	(!pthread_mutex_trylock(&(l)->lock))
#define mutex_unlock(l)		pthread_mutex_unlock(&(l)->lock)

#endif /* __TOOLS_LINUX_MUTEX_H */
