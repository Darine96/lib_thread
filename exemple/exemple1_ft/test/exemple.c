#include "fthread.h"
#include "stdio.h"
#include "time.h"

/* stop */

void pr (void *text)
{
   while (1) {
      fprintf(stdout,"%s",(char*)text);
      ft_thread_cooperate ();
   }
}

void control (void* args)
{
   ft_thread_cooperate_n (10);
   ft_scheduler_stop ((ft_thread_t)args);
   fprintf (stdout,"stop\n");
   ft_thread_join ((ft_thread_t)args);
   fprintf (stdout,"exit\n");
   exit (0);
}

void traceInstants ()
{
   int i = 0;
   while (1) {
      fprintf (stdout,">>>>>>>>>>> instant %d\n",i++);
      ft_thread_cooperate ();
   }
}

int main (void)
{
	/* begin time */
  clock_t tic = clock();
  ft_thread_t ft;
  ft_scheduler_t sched = ft_scheduler_create ();

  ft_thread_create (sched,traceInstants,NULL,NULL);

  ft = ft_thread_create (sched,pr,NULL,"*");
  ft_thread_create (sched,control,NULL,(void*)ft);

  ft_scheduler_start (sched);
  ft_exit ();
fprintf (stdout,"exit\n");
  /* end time */
  clock_t toc = clock();
  fprintf (stdout,"Elapsed: %f seconds\n",(double)(toc - tic) / CLOCKS_PER_SEC);
  //printf("Elapsed: %f seconds\n", (double)(toc - tic) / CLOCKS_PER_SEC);
  return 0;
}

