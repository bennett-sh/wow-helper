import requests

print("fetching wordlist...")
wordlist_response = requests.get('https://github.com/davidak/wortliste/raw/master/wortliste.txt')
print("fetched wordlist. saving...")

with open('./wordlist.txt', 'w', encoding='utf-8') as wordlist:
  wordlist.write(wordlist_response.text)

print("done.")
