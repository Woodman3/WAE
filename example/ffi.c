#include<stdio.h>
#include<stdint.h>
#include<string.h>

extern uint8_t init(char* path);
extern uint8_t step();
extern char* get_obs();
extern char* get_acs();
extern uint8_t action(char* args);
extern uint8_t free_str(char* str);

int main(){
    uint8_t success = init("../config"); 
    if(!success){
        char buffer[1000];
        char* obs = get_obs();
        strcpy(buffer,obs);
        uint8_t r =free_str(obs);
        printf("r=%d\n%s",r,buffer);
    }else{
        printf("wrong in init()");
    }
    return 0;
}