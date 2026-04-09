#include <girepository/girepository.h>

int main(void) {
    GIRepository *repository = g_irepository_get_default();
    return repository == NULL;
}

