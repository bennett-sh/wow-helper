import requests

def update(src: str):
  print("fetching wordlist...")
  wordlist_response = requests.get(src)
  print("fetched wordlist. saving...")

  with open("./wordlist.txt", "w", encoding="utf-8") as wordlist:
    wordlist.write(wordlist_response.text)

  print("done.")
