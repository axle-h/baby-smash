#include "VideoScaler.h"
#include <stdexcept>

AVFrame *allocateFrame(int width, int height) {
  auto frame = av_frame_alloc();
  frame->format = AV_PIX_FMT_YUV420P;
  frame->width = width;
  frame->height = height;
  if (av_frame_get_buffer(frame, 0) < 0) {
    throw std::runtime_error("Cannot allocate scaled frame");
  }
  return frame;
}

VideoScaler::VideoScaler(const VideoFormat &sourceFormat, int scaledWidth, int scaledHeight)
    : sourceFormat(sourceFormat), scaledWidth(scaledWidth), scaledHeight(scaledHeight) {
  setSourceFormat(sourceFormat);
  scaledFrame = allocateFrame(scaledWidth, scaledHeight);
}

void VideoScaler::setSourceFormat(const VideoFormat &sourceFormat) {
  VideoScaler::sourceFormat = sourceFormat;
  swsContext = sws_getContext(
    sourceFormat.width,
    sourceFormat.height,
    sourceFormat.pixelFormat,
    scaledWidth,
    scaledHeight,
    AV_PIX_FMT_YUV420P,
    SWS_BICUBIC,
    nullptr, nullptr, nullptr);
}

VideoScaler::~VideoScaler() {
  sws_freeContext(swsContext);
  av_frame_free(&scaledFrame);
}

void VideoScaler::scale(AVFrame *frame) const {
  if (sws_scale(swsContext, frame->data, frame->linesize, 0, frame->height, scaledFrame->data, scaledFrame->linesize) < 0) {
    throw std::runtime_error("Cannot scale frame");
  }
}

AVFrame *VideoScaler::getScaledFrame() const {
  return scaledFrame;
}
