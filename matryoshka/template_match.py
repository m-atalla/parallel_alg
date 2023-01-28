import cv2 
from matplotlib import pyplot as plt
import numpy as np
import multiprocessing as mp
from functools import partial


def read_images():
    img = cv2.imread('l.png')
    img_gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    template = cv2.imread('k.png',0)

    return (img, img_gray, template)

def template_match_seq():
    img, img_gray, template = read_images()


    result = cv2.matchTemplate(img_gray, template, cv2.TM_CCOEFF)

    img = highlight_result(img, template, result)

    # Write result
    cv2.imwrite('result_seq.png', img)

def template_match_par():
    img, img_gray, template = read_images()
    cores = mp.cpu_count()
    sub_imgs = np.array_split(img_gray, cores)
    results = []
    with mp.Pool(processes=cores) as pool:
        results = pool.map(partial(cv2.matchTemplate, templ=template, method=cv2.TM_CCOEFF),  sub_imgs)
        pool.close()
        pool.join()


    img = highlight_result(img, template, np.concatenate(results))

    # Write result
    cv2.imwrite('result_par.png', img)

def highlight_result(original_img, template, result):
    template_w, template_h = template.shape[::-1]
    _, _, _, max_loc = cv2.minMaxLoc(result)
    top_left = max_loc

    # Calculates bottom right coordinates
    bottom_right = (top_left[0] + template_w, top_left[1] + template_h)

    # Draw rectangle at the original RGB image
    cv2.rectangle(original_img, top_left, bottom_right, (0,255,0), 2)

    return original_img

def main():
    template_match_par()

if __name__ == '__main__':
    main()
