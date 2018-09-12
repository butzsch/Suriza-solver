import cv2
import importlib.util
import numpy as np
import time

from tesserocr import PyTessBaseAPI, PSM

spec = importlib.util.spec_from_file_location("libsuriza", "target/release/libsuriza.so")
libsuriza = importlib.util.module_from_spec(spec)
spec.loader.exec_module(libsuriza)

IMAGE_WIDTH = 1024
IMAGE_HEIGHT = 768

def without_fisheye(image):
    DIM = (IMAGE_WIDTH, IMAGE_HEIGHT)
    K = np.array([[897.8161229799011, 0.0, 528.2264984903634], [0.0, 900.0479778698168, 384.0738210318443], [0.0, 0.0, 1.0]])
    D = np.array([[-0.14402053307550466], [0.6010714757170741], [-3.287858742663824], [5.940476372000673]])
    
    map1, map2 = cv2.fisheye.initUndistortRectifyMap(K, D, np.eye(3), K, DIM, cv2.CV_16SC2)
    
    return cv2.remap(image, map1, map2, interpolation=cv2.INTER_LINEAR, borderMode=cv2.BORDER_CONSTANT)

def capture_image():
    camera = cv2.VideoCapture(0)
    
    camera.set(cv2.CAP_PROP_FRAME_WIDTH, IMAGE_WIDTH)
    camera.set(cv2.CAP_PROP_FRAME_HEIGHT, IMAGE_HEIGHT)

    while True:
        retval, image = camera.read()
        if retval:
            return image

grbl = libsuriza.GRBL('/dev/serial/by-id/usb-1a86_USB2.0-Serial-if00-port0')

image = capture_image()
undistorted = without_fisheye(image.copy())
excerpt = undistorted[215:525, 200:800]

CELL_COUNT = 7
INTERSECTION_COUNT = CELL_COUNT + 1

gray_image = cv2.cvtColor(excerpt, cv2.COLOR_BGR2GRAY)
gray_image = cv2.adaptiveThreshold(gray_image, 255, cv2.ADAPTIVE_THRESH_GAUSSIAN_C, cv2.THRESH_BINARY, 11, 10)

_, contours, hierarchy = cv2.findContours(gray_image, cv2.RETR_CCOMP, cv2.CHAIN_APPROX_SIMPLE)
contours = [contour for i, contour in enumerate(contours) if hierarchy[0][i][3] != -1]
contours = [contour for contour in contours if cv2.contourArea(contour) < 100]

rects = sorted([cv2.boundingRect(contour) for contour in contours], key = lambda rect: rect[1])

first_row, last_row = rects[:INTERSECTION_COUNT], rects[-INTERSECTION_COUNT:]

first_row.sort(key = lambda rect: rect[0])
last_row.sort(key = lambda rect: rect[0])

top_left, top_right = first_row[0], first_row[-1]
bottom_left, bottom_right = last_row[0], last_row[-1]

TARGET_INTERSECTION_SIZE = 32
TARGET_CELL_SIZE = 96
TARGET_SIZE = CELL_COUNT * (TARGET_INTERSECTION_SIZE + TARGET_CELL_SIZE)

source = np.float32([top_left[:2], top_right[:2], bottom_left[:2], bottom_right[:2]])
target = np.float32([(0, 0), (TARGET_SIZE, 0), (0, TARGET_SIZE), (TARGET_SIZE, TARGET_SIZE)])
transform = cv2.getPerspectiveTransform(source, target)

warped = cv2.warpPerspective(gray_image, transform, (TARGET_SIZE, TARGET_SIZE))

def detect_digits():
    with PyTessBaseAPI(psm = PSM.SINGLE_CHAR) as api:
        api.SetVariable("tessedit_char_whitelist", "0123")

        def map_cell(row, column):
            x = column * (TARGET_INTERSECTION_SIZE + TARGET_CELL_SIZE) + TARGET_INTERSECTION_SIZE
            y = row * (TARGET_INTERSECTION_SIZE + TARGET_CELL_SIZE) + TARGET_INTERSECTION_SIZE

            cell_image = warped[y:y + TARGET_CELL_SIZE, x:x + TARGET_CELL_SIZE]
            
            api.SetImageBytes(cell_image.tostring(), TARGET_CELL_SIZE, TARGET_CELL_SIZE, 1, TARGET_CELL_SIZE)
            
            return api.GetUTF8Text()[0]

        def map_row(row):
            return [map_cell(row, column) for column in range(0, CELL_COUNT)]

        return [map_row(row) for row in range(0, CELL_COUNT)]
    
grid = detect_digits()

START_X, START_Y = 55, 131
grbl.seek_to(START_X, START_Y)
time.sleep(1)

EDGE_LENGTH = 6.5

route = libsuriza.solve(grid)
grbl.lower_pen()

for column, row in route:
    x = START_X + EDGE_LENGTH * column
    y = START_Y - EDGE_LENGTH * row
    
    grbl.move_to(x, y)

grbl.raise_pen()
grbl.seek_to(0, 0)
grbl.lower_pen()

time.sleep(5)
