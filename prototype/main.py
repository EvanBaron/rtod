import cv2
from ultralytics.models import yolo

# Load YOLOv8 Nano model
model = yolo.YOLO("prototype/yolov8n.pt")

# Open webcam
cap = cv2.VideoCapture(0)
while cap.isOpened():
    success, frame = cap.read()
    if not success:
        break

    # Pass the frame to the AI
    results = model(frame)

    # Bounding boxes and labels directly onto the frame
    annotated_frame = results[0].plot()

    # Display
    cv2.imshow("Live AI Vision", annotated_frame)

    if cv2.waitKey(1) & 0xFF == ord("q"):
        break

# Clean up hardware resources
cap.release()
cv2.destroyAllWindows()
