NAME= Futura

build:
    GOARCH=amd64 GOOS=windows go build -o ${NAME}-windows main.go

run: build
    ./$(NAME)

clean:
    go clean

    rm $(NAME)-windows
