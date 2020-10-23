import sys
import platform
import tarfile
import urllib.request


def main():
    out_dir = sys.argv[1]

    arch = platform.system()
    if arch == "Linux":
        arch = "linux-64"
    elif arch == "Darwin":
        arch = "osx-64"
    elif arch == "Windows":
        arch = "win-64"
    else:
        raise RuntimeError("platform {!r} not supported".format(arch))

    url = "https://anaconda.org/intel/mkl-static/2020.0/download/{}/mkl-static-2020.0-intel_166.tar.bz2".format(arch)
    archive = "{}/mkl-static.tar.bz2".format(out_dir)

    req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
    with open(archive, "wb") as fh:
        fh.write(urllib.request.urlopen(req).read())

    with tarfile.open(archive, "r:bz2") as fh:
        fh.extractall(out_dir)


if __name__ == "__main__":
    main()