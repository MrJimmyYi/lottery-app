# Makefile

# 项目名称
PROJECT_NAME := op_excel

# Windows平台构建命令
build-windows:
	GOOS=windows GOARCH=amd64 go build  -v -ldflags="-w" -o $(PROJECT_NAME)_windows.exe main.go

# macOS平台构建命令
build-mac:
	GOOS=darwin GOARCH=amd64 go build -v -ldflags="-w"  -o $(PROJECT_NAME)_mac main.go

# Linux平台构建命令
build-linux:
	GOOS=linux GOARCH=amd64 go build -v -ldflags="-w" -o $(PROJECT_NAME)_linux main.go

# 构建所有平台的二进制文件
build-all: build-windows build-mac build-linux

# 清除构建文件
clean:
	rm -f $(PROJECT_NAME)-windows.exe $(PROJECT_NAME)-mac $(PROJECT_NAME)-linux

.PHONY: build-windows build-mac build-linux build-all clean
