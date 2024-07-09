from pytube import YouTube
#Fetches meta data of video and downloads video
def download_n_fetchmeta (url):
    query = []
    yt = YouTube(url)
    query.append(yt.thumbnail_url)
    query.append(yt.title)
    yt.streams.first().download(output_path='./download/')
    return query

