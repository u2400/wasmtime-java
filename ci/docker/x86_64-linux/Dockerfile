FROM centos:7

ARG TOOLCHAIN=nightly-2023-09-01

RUN rm -rf /root/.rustup /root/.cargo 

RUN sed -i 's/mirrorlist/#mirrorlist/g' /etc/yum.repos.d/CentOS-*.repo && \
    sed -i 's|#baseurl=http://mirror.centos.org|baseurl=http://vault.centos.org|g' /etc/yum.repos.d/CentOS-*.repo && \
    yum install -y git gcc java-11-openjdk

ENV PATH=$PATH:/rust/bin:/root/.cargo/bin
ENV JAVA_HOME=/etc/alternatives/jre_11_openjdk

# Confirm that the JAVA_HOME var is set correctly
RUN ls ${JAVA_HOME}/bin/java

# Install rustup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=${TOOLCHAIN} --profile=minimal && rustc --version
RUN rustup component add rustfmt
