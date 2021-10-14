import requests
import json

def getDingMes():

    baseUrl = "https://oapi.dingtalk.com/robot/send?access_token=efcb0331f064f659d8ee966102c69eb4efea55ca22b4c86a7013f52aeac91cd0"

    # please set charset= utf-8
    HEADERS = {
        "Content-Type": "application/json ;charset=utf-8 "
    }

# 这里的message是你想要推送的文字消息
    message = "my test message"
    stringBody ={
        "msgtype": "text",
        "text": {"content": message},
        "at": {
            "atMobiles": ["13951026738"],
            "atUserIds": ["周义其"],
            "isAtAll": False   #@所有人 时为true，上面的atMobiles就失效了
        }
 }
    MessageBody = json.dumps(stringBody)
    result = requests.post(url=baseUrl, data=MessageBody, headers=HEADERS)
    print(result.text)

if __name__ == '__main__':
    getDingMes()