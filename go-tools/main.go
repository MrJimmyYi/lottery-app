package main

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"github.com/xuri/excelize/v2"
	"log"
	"os"
	"path/filepath"
	"strings"
)

const IMG_INDXE_EX = 4

// UserInfo 结构体定义
type UserInfo struct {
	Num    string `json:"num"`
	Name   string `json:"name"`
	Img    string `json:"img"`
	Remark string `json:"remark"`
}

func main() {
	exfile := os.Args[1]
	imgDir := os.Args[2]
	//filename := "Book1.xlsx"
	//imgDir := "/Users/jimmy/Desktop/Src/Workspace/op_excel"
	f, err := excelize.OpenFile(exfile)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	defer func() {
		if err := f.Close(); err != nil {
			fmt.Println(err)
		}
	}()

	// 假设你的数据在名为"Sheet1"的工作表中
	sheetName := "data"

	// 获取指定工作表的所有行
	rows, err := f.GetRows(sheetName)
	if err != nil {
		log.Fatalf("Failed to get rows: %v", err)
		os.Exit(1)
	}

	// 创建一个空的Person类型的切片
	userInfoArray := make([]UserInfo, 0)

	for rowIndex, row := range rows {
		if rowIndex == 0 {
			continue
		}
		num, name, remark := strings.TrimSpace(row[0]), strings.TrimSpace(row[1]), strings.TrimSpace(row[2])
		img := ""
		pictures, err := f.GetPictures(sheetName, getExcelColumnName(IMG_INDXE_EX)+fmt.Sprint(rowIndex+1))
		if err != nil {
			fmt.Println("Error getting picture:", err)
		} else {
			imgName := fmt.Sprintf("%s.png", generateRandomHash(16))
			img = filepath.Join(imgDir, imgName)
			if err := os.WriteFile(img, pictures[0].File, 0644); err != nil {
				fmt.Println(err)
			}
		}
		user := UserInfo{num, name, img, remark}
		userInfoArray = append(userInfoArray, user)
	}
	// 序列化people切片为JSON字符串
	jsonData, err := json.Marshal(userInfoArray)
	if err != nil {
		os.Exit(1)
	}

	// 将JSON字节切片转换为字符串并打印
	jsonString := string(jsonData)
	fmt.Println(jsonString)

}

// GetExcelColumnName 根据列索引返回Excel列名（例如，1 -> "A", 28 -> "AB"）
func getExcelColumnName(index int) string {
	columnName := ""
	for index > 0 {
		mod := (index - 1) % 26
		columnName = string('A'+mod) + columnName
		index = (index - mod) / 26
	}
	return columnName
}

// generateRandomHash 生成一个指定长度的随机哈希值。
func generateRandomHash(num int) string {
	randomBytes := make([]byte, num)
	_, err := rand.Read(randomBytes)
	if err != nil {
		return "ABCDEFGHIJKLMNOP"
	}
	return hex.EncodeToString(randomBytes)
}
