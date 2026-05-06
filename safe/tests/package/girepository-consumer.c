#include <girepository/girepository.h>

int main(void) {
    GIRepository *repository = gi_repository_new();
    return repository == NULL;
}
