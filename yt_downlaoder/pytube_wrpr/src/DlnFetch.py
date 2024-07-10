from pytube import YouTube
#Fetches meta data of video and downloads video
def download_n_fetchmeta (url,dir):
    query = []
    yt = YouTube(url)
    query.append(yt.thumbnail_url)
    query.append(yt.title)
    yt.streams.first().download(output_path=dir)
    return query

if __name__ == '__main__':
 download_n_fetchmeta("http://youtube.com/watch?v=2lAe1cqCOXo","../download/")
